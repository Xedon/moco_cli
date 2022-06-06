use std::{cell::RefCell, error::Error, io::Write, sync::Arc, vec};

use crate::{
    moco::client::MocoClient,
    utils::{ask_question, mandatory_validator, optional_validator},
};

use chrono::Utc;
use jira_tempo::client::JiraTempoClient;
use log::trace;
use utils::render_table;

use crate::moco::model::{CreateActivitie, DeleteActivitie};

mod cli;
mod config;
mod jira_tempo;
mod moco;
mod tempo;

mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::init();
    let mut log_builder = env_logger::builder();
    log_builder.parse_default_env();
    if args.debug {
        log_builder.filter_level(log::LevelFilter::Trace);
    }
    log_builder.init();
    let config = Arc::new(RefCell::new(config::init()?));
    let moco_client = MocoClient::new(&config);
    let tempo_client = JiraTempoClient::new(&config);

    match args.command {
        cli::Commands::Login { system } => match system {
            cli::Login::Jira => {
                println!("Jira Tempo Login");

                let api_key = ask_question("Enter your personal api key: ", &mandatory_validator)?;
                config.borrow_mut().jira_tempo_api_key = Some(api_key);

                tempo_client.test_login().await?;

                config.borrow_mut().write_config()?;
                println!("🤩 Logged in 🤩")
            }
            cli::Login::Moco => {
                println!("Moco Login");

                let moco_company = ask_question("Enter moco company name: ", &mandatory_validator)?;
                let api_key = ask_question("Enter your personal api key: ", &mandatory_validator)?;

                config.borrow_mut().moco_company = Some(moco_company);
                config.borrow_mut().moco_api_key = Some(api_key);

                let firstname = ask_question("Enter firstname: ", &mandatory_validator)?;
                let lastname = ask_question("Enter lastname:  ", &mandatory_validator)?;

                let client_id = moco_client.get_user_id(firstname, lastname).await?;

                config.borrow_mut().moco_user_id = client_id;
                config.borrow_mut().write_config()?;
                println!("🤩 Logged in 🤩")
            }
        },
        cli::Commands::List { today, week, month } => {
            let (from, to) = utils::select_from_to_date(today, week || !today && !month, month);

            let activities = moco_client
                .get_activities(
                    from.format("%Y-%m-%d").to_string(),
                    to.format("%Y-%m-%d").to_string(),
                    None,
                    None,
                )
                .await?;

            let mut list: Vec<Vec<String>> = activities
                .iter()
                .map(|activity| {
                    vec![
                        activity.customer.name.clone(),
                        activity.task.name.clone(),
                        activity.date.clone(),
                        activity.hours.to_string(),
                        activity
                            .description
                            .as_ref()
                            .unwrap_or(&String::new())
                            .to_string(),
                    ]
                })
                .collect();
            list.insert(
                0,
                vec![
                    "Customer".to_string(),
                    "Task".to_string(),
                    "Date".to_string(),
                    "Hours".to_string(),
                    "Description".to_string(),
                ],
            );

            list.push(vec![
                "-".to_string(),
                "-".to_string(),
                "-".to_string(),
                activities
                    .iter()
                    .fold(0.0, |hours, activity| activity.hours + hours)
                    .to_string(),
                "".to_string(),
            ]);

            render_table(list);
        }
        cli::Commands::New {
            project,
            task,
            hours,
            date,
            description,
        } => {
            let now = Utc::now().format("%Y-%m-%d").to_string();

            let (project, task) = promp_task_select(&moco_client, project, task).await?;

            let date = if let Some(d) = date {
                d
            } else {
                print!("Date (YYYY-mm-DD) default ({}): ", now);
                std::io::stdout().flush()?;

                let date = utils::read_line()?;
                if date.is_empty() {
                    now
                } else {
                    date
                }
            };

            let hours = if let Some(h) = hours {
                h
            } else {
                ask_question("Time in Hours: ", &|answer| {
                    answer.parse::<f64>().err().map(|e| format!("{}", e))
                })?
                .parse::<f64>()?
            };

            let description = if let Some(d) = description {
                d
            } else {
                ask_question("Description: ", &optional_validator)?
            };

            moco_client
                .create_activitie(&CreateActivitie {
                    date,
                    project_id: project.id,
                    task_id: task.id,
                    hours: Some(hours),
                    description,
                    ..Default::default()
                })
                .await?;
        }
        cli::Commands::Add => println!("not yet implemented"),
        cli::Commands::Edit => println!("not yet implemented"),
        cli::Commands::Rm { activity } => {
            let activity = promp_activitie_select(&moco_client, activity).await?;

            moco_client
                .delete_activitie(&DeleteActivitie {
                    activity_id: activity.id,
                    ..Default::default()
                })
                .await?;
        },
        cli::Commands::Sync {
            system,
            today,
            week,
            month,
            dry_run,
            project,
            task,
        } => match system {
            cli::Sync::Jira => {
                let (from, to) = utils::select_from_to_date(today, week, month);

                let worklogs = tempo_client
                    .get_worklogs(
                        from.format("%Y-%m-%d").to_string(),
                        to.format("%Y-%m-%d").to_string(),
                    )
                    .await?;

                trace!("Tempo: {:#?}", worklogs);

                let (project, task) = promp_task_select(&moco_client, project, task).await?;

                let activities = moco_client
                    .get_activities(
                        from.format("%Y-%m-%d").to_string(),
                        to.format("%Y-%m-%d").to_string(),
                        Some(task.id.to_string()),
                        Some("mococli".to_string()),
                    )
                    .await?;

                let worklogs: Vec<Result<CreateActivitie, Box<dyn Error>>> = worklogs
                    .results
                    .iter()
                    .filter(|worklog| {
                        !activities.iter().any(|activity| {
                            activity
                                .remote_id
                                .as_ref()
                                .and_then(|x| x.parse::<i64>().ok())
                                .unwrap_or(0)
                                == worklog.jira_worklog_id
                        })
                    })
                    .map(|worklog| -> Result<CreateActivitie, Box<dyn Error>> {
                        Ok(CreateActivitie {
                            remote_service: Some("jira".to_string()),
                            seconds: Some(worklog.time_spent_seconds),
                            date: worklog.start_date.to_string(),
                            tag: Some("mococli".to_string()),
                            project_id: project.id,
                            task_id: task.id,
                            description: worklog.description.clone(),
                            remote_id: Some(worklog.jira_worklog_id.to_string()),
                            ..Default::default()
                        })
                    })
                    .collect();

                let output_list = vec!["Date", "Hours", "Description", "Project ID", "Task ID"];

                let mut output_list = vec![output_list.iter().map(|str| str.to_string()).collect()];

                for worklog in &worklogs {
                    if let Ok(worklog) = &worklog {
                        output_list.push(vec![
                            worklog.date.clone(),
                            worklog
                                .seconds
                                .map(|seconds| seconds as f64 / 60.0 / 60.0)
                                .unwrap_or(0.0)
                                .to_string(),
                            worklog.description.clone(),
                            worklog.project_id.to_string(),
                            worklog.task_id.to_string(),
                        ])
                    }
                    if let Err(err) = &worklog {
                        output_list.push(vec![
                            "Error".to_string(),
                            format!("{:?}", err),
                            "".to_string(),
                            "".to_string(),
                            "".to_string(),
                        ])
                    }
                }

                if dry_run {
                    println!("Planed sync: ");
                    println!(
                        "From {} to {}",
                        from.format("%d.%m.%y"),
                        to.format("%d.%m.%y")
                    );
                    if output_list.len() == 1 {
                        print!("Nothing, everything seems to be Synced!")
                    } else {
                        render_table(output_list);
                    }
                    println!();
                } else {
                    println!("Sync plan: ");
                    println!(
                        "From {} to {}",
                        from.format("%d.%m.%y"),
                        to.format("%d.%m.%y")
                    );
                    if output_list.len() == 1 {
                        print!("Nothing, everything seems to be Synced!")
                    } else {
                        render_table(output_list);
                    }

                    println!();

                    for worklog in worklogs {
                        if let Ok(worklog) = &worklog {
                            moco_client.create_activitie(worklog).await?;
                        }
                    }
                    println!("Synced!");
                }
            }
        },
    }

    Ok(())
}

async fn promp_task_select(
    moco_client: &MocoClient,
    project: Option<i64>,
    task: Option<i64>,
) -> Result<(moco::model::Project, moco::model::ProjectTask), Box<dyn Error>> {
    let projects = moco_client.get_assigned_projects().await?;
    let project = projects.iter().find(|p| p.id == project.unwrap_or(-1));

    let project = if let Some(p) = project {
        p
    } else {
        let project_index = utils::render_list_select(
            &projects,
            vec!["Index", "Customer", "Project", "Project ID"],
            "Chose your Project: ",
            &(|(index, project)| {
                vec![
                    index.to_string(),
                    project.customer.name.clone(),
                    project.name.clone(),
                    project.id.to_string(),
                ]
            }),
        )?;

        &projects[project_index]
    };

    let task = project.tasks.iter().find(|t| t.id == task.unwrap_or(-1));

    let task = if let Some(t) = task {
        t
    } else {
        let task_index = utils::render_list_select(
            &project.tasks,
            vec!["Index", "Task", "Task ID"],
            "Chose your Task: ",
            &(|(index, task)| vec![index.to_string(), task.name.clone(), task.id.to_string()]),
        )?;
        &project.tasks[task_index]
    };

    Ok((project.clone(), task.clone()))
}

async fn promp_activitie_select(
    moco_client: &MocoClient,
    activity: Option<i64>,
) -> Result<moco::model::Activitie, Box<dyn Error>> {
    let now = Utc::now().format("%Y-%m-%d").to_string();

    print!("List activities from (YYYY-MM-DD) - Default 'today': ");
    std::io::stdout().flush()?;

    let mut from = utils::read_line()?;
    if from.is_empty() {
        from = now.clone();
    }

    print!("List activities to (YYYY-MM-DD) - Default 'last answer': ");
    std::io::stdout().flush()?;

    let mut to = utils::read_line()?;
    if to.is_empty() {
        to = from.clone();
    }

    let activities = moco_client.get_activities(from, to, None, None).await?;
    let activity = activities.iter().find(|a| a.id == activity.unwrap_or(-1));

    let activity = if let Some(a) = activity {
        a
    } else {
        let activity_index = utils::render_list_select(
            &activities,
            vec!["Index", "Date", "Duration", "Project", "Task", "Description"],
            "Choose your Acitivity: ",
            &(|(index, activity)| {
                vec![
                    index.to_string(),
                    activity.date.clone(),
                    activity.hours.to_string(),
                    activity.project.name.clone(),
                    activity.task.name.clone(),
                    activity.description
                        .as_ref()
                        .unwrap_or(&String::new())
                        .to_string()
                ]
            }),
        )?;

        &activities[activity_index]
    };

    Ok(activity.clone())
}
