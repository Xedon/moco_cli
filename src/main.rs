use std::{cell::RefCell, error::Error, io::Write, sync::Arc, vec};

use chrono::Utc;
use log::trace;

use jira_tempo::client::JiraTempoClient;
use utils::{promp_activity_select, promp_task_select, render_table};

use crate::moco::model::{ControlActivityTimer, CreateActivity, DeleteActivity, GetActivity};
use crate::{
    moco::{client::MocoClient, model::EditActivity},
    utils::{ask_question, mandatory_validator, optional_validator},
};

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

                let api_key = ask_question("Enter your personal API key: ", &mandatory_validator)?;
                config.borrow_mut().jira_tempo_api_key = Some(api_key);

                tempo_client.test_login().await?;

                config.borrow_mut().write_config()?;
                println!("🤩 Logged in 🤩")
            }
            cli::Login::Moco => {
                println!("Moco Login");

                let moco_company = ask_question("Enter Moco company name: ", &mandatory_validator)?;
                let api_key = ask_question("Enter your personal API key: ", &mandatory_validator)?;

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
                        activity.date.clone(),
                        activity.hours.to_string(),
                        activity.customer.name.clone(),
                        activity.task.name.clone(),
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
                    "Date".to_string(),
                    "Duration (hours)".to_string(),
                    "Customer".to_string(),
                    "Task".to_string(),
                    "Description".to_string(),
                ],
            );

            list.push(vec![
                "-".to_string(),
                activities
                    .iter()
                    .fold(0.0, |hours, activity| activity.hours + hours)
                    .to_string(),
                "-".to_string(),
                "-".to_string(),
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
                print!("Date (YYYY-MM-DD) - Default 'today': ");
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
                let answer =
                    ask_question("Duration (hours) - Default 'start timer': ", &|answer| {
                        answer
                            .is_empty()
                            .then(|| None)
                            .unwrap_or(answer.parse::<f64>().err().map(|e| format!("{}", e)))
                    })?;
                answer
                    .is_empty()
                    .then(|| 0_f64)
                    .unwrap_or_else(|| answer.parse::<f64>().unwrap())
            };

            let description = if let Some(d) = description {
                d
            } else {
                ask_question("Description: ", &optional_validator)?
            };

            moco_client
                .create_activity(&CreateActivity {
                    date,
                    project_id: project.id,
                    task_id: task.id,
                    hours: Some(hours),
                    description,
                    ..Default::default()
                })
                .await?;
        }
        cli::Commands::Edit { activity } => {
            let activity = promp_activity_select(&moco_client, activity).await?;

            let now = Utc::now().format("%Y-%m-%d").to_string();

            print!("New date (YYYY-MM-DD) - Default '{}': ", activity.date);
            std::io::stdout().flush()?;

            let mut date = utils::read_line()?;
            if date.is_empty() {
                date = now.clone()
            }

            print!("New duration (hours) - Default '{}': ", activity.hours);
            std::io::stdout().flush()?;

            let mut hours = utils::read_line()?;
            if hours.is_empty() {
                hours = activity.hours.to_string()
            }

            print!("New description - Default 'current': ");
            std::io::stdout().flush()?;

            let mut description = utils::read_line()?;
            if description.is_empty() {
                description = activity
                    .description
                    .as_ref()
                    .unwrap_or(&String::new())
                    .to_string()
            }

            moco_client
                .edit_activity(&EditActivity {
                    activity_id: activity.id,
                    project_id: activity.project.id,
                    task_id: activity.task.id,
                    date,
                    description,
                    hours,
                })
                .await?;
        }
        cli::Commands::Rm { activity } => {
            let activity = promp_activity_select(&moco_client, activity).await?;

            moco_client
                .delete_activity(&DeleteActivity {
                    activity_id: activity.id,
                })
                .await?;
        }
        cli::Commands::Timer { system, activity } => match system {
            cli::Timer::Start => {
                let activity = promp_activity_select(&moco_client, activity).await?;

                moco_client
                    .control_activity_timer(&ControlActivityTimer {
                        control: "start".to_string(),
                        activity_id: activity.id,
                    })
                    .await?;
            }
            cli::Timer::Stop => {
                let now = Utc::now().format("%Y-%m-%d").to_string();
                let from = now.clone();
                let to = now.clone();

                let activities = moco_client.get_activities(from, to, None, None).await?;
                let activity = activities.iter().find(|a| !a.timer_started_at.is_null());

                if let Some(a) = activity {
                    moco_client
                        .control_activity_timer(&ControlActivityTimer {
                            control: "stop".to_string(),
                            activity_id: a.id,
                        })
                        .await?;

                    let a = moco_client
                        .get_activity(&GetActivity { activity_id: a.id })
                        .await?;
                    println!("Activity duration: {} hours", a.hours);
                } else {
                    println!("Could not stop timer since it was not on");
                }
            }
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

                let worklogs: Vec<Result<CreateActivity, Box<dyn Error>>> = worklogs
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
                    .map(|worklog| -> Result<CreateActivity, Box<dyn Error>> {
                        Ok(CreateActivity {
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

                let output_list = vec![
                    "Date",
                    "Duration (hours)",
                    "Description",
                    "Project ID",
                    "Task ID",
                ];

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
                            moco_client.create_activity(worklog).await?;
                        }
                    }
                    println!("Synced!");
                }
            }
        },
    }

    Ok(())
}
