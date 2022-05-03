use std::{cell::RefCell, error::Error, io::Write, sync::Arc, vec};

use crate::{moco::client::MocoClient, utils::ask_question};

use chrono::Utc;
use jira_tempo::client::JiraTempoClient;
use utils::render_table;

use crate::moco::model::CreateActivitie;

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
                print!("Enter your personal api key: ");
                std::io::stdout().flush()?;

                let api_key = utils::read_line()?;
                config.borrow_mut().jira_tempo_api_key = Some(api_key);

                tempo_client.test_login().await?;

                config.borrow_mut().write_config()?;
                println!("🤩 Logged in 🤩")
            }
            cli::Login::Moco => {
                println!("Moco Login");

                print!("Enter moco company name: ");
                std::io::stdout().flush()?;
                let moco_company = utils::read_line()?;
                config.borrow_mut().moco_company = Some(moco_company);

                print!("Enter your personal api key: ");
                std::io::stdout().flush()?;

                let api_key = utils::read_line()?;
                config.borrow_mut().moco_api_key = Some(api_key);

                print!("Enter firstname: ");
                std::io::stdout().flush()?;
                let firstname = utils::read_line()?;

                print!("Enter lastname:  ");
                std::io::stdout().flush()?;
                let lastname = utils::read_line()?;

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

            render_table(list);
        }
        cli::Commands::New => {
            let now = Utc::now().format("%Y-%m-%d").to_string();

            let (project, task) = promp_task_select(&moco_client).await?;

            print!("Date (YYYY-mm-DD) default ({}): ", now);
            std::io::stdout().flush()?;

            let mut date = utils::read_line()?;
            if date.is_empty() {
                date = now;
            }

            let hours = ask_question("Time in Hours: ", &|answer| {
                answer.parse::<f64>().err().map(|e| format!("{}", e))
            })?;

            let description = ask_question("Description: ", &|_| None)?;

            moco_client
                .create_activitie(&CreateActivitie {
                    date,
                    project_id: project.id,
                    task_id: task.id,
                    hours: Some(hours.parse::<f64>()?),
                    description,
                    ..Default::default()
                })
                .await?;
        }
        cli::Commands::Add => println!("not yet implemented"),
        cli::Commands::Edit => println!("not yet implemented"),
        cli::Commands::Rm => println!("not yet implemented"),
        cli::Commands::Sync {
            system,
            today,
            week,
            month,
            dry_run,
        } => match system {
            cli::Sync::Jira => {
                let (from, to) = utils::select_from_to_date(today, week, month);

                let worklogs = tempo_client
                    .get_worklogs(
                        from.format("%Y-%m-%d").to_string(),
                        to.format("%Y-%m-%d").to_string(),
                    )
                    .await?;

                let (project, task) = promp_task_select(&moco_client).await?;

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
                            date: chrono::DateTime::parse_from_rfc3339(
                                &worklog.created_at.to_string(),
                            )?
                            .format("%Y-%m-%d")
                            .to_string(),
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
                                .map(|x| x as f64 / 60.0 / 60.0)
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
                    print!("Planed sync: ");
                    if output_list.len() == 1 {
                        print!("Nothing, everything seems to be Synced!")
                    } else {
                        render_table(output_list);
                    }
                    println!();
                } else {
                    print!("Sync plan: ");
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
) -> Result<(moco::model::Project, moco::model::ProjectTask), Box<dyn Error>> {
    let projects = moco_client.get_assigned_projects().await?;

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
    let project = &projects[project_index];
    let task_index = utils::render_list_select(
        &project.tasks,
        vec!["Index", "Task", "Task ID"],
        "Chose your Task: ",
        &(|(index, task)| vec![index.to_string(), task.name.clone(), task.id.to_string()]),
    )?;
    let task = &project.tasks[task_index];
    Ok((project.clone(), task.clone()))
}
