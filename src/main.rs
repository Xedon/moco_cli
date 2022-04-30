use std::{cell::RefCell, error::Error, io::Write, sync::Arc};

use crate::moco::client::MocoClient;
use chrono::Utc;
use jira_tempo::client::JiraTempoClient;

use crate::moco::model::CreateActivitie;

use now::DateTimeNow;
mod cli;
mod config;
mod jira_tempo;
mod moco;
mod tempo;

fn read_line() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    input.remove(input.len() - 1);
    Ok(input)
}

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

                let api_key = read_line()?;
                config.borrow_mut().jira_tempo_api_key = Some(api_key);

                tempo_client.test_login().await?;

                config.borrow_mut().write_config()?;
                println!("🤩 Logged in 🤩")
            }
            cli::Login::Moco => {
                println!("Moco Login");
                print!("Enter your personal api key: ");
                std::io::stdout().flush()?;

                let api_key = read_line()?;
                config.borrow_mut().moco_api_key = Some(api_key);

                print!("Enter firstname: ");
                std::io::stdout().flush()?;
                let firstname = read_line()?;

                print!("Enter lastname:  ");
                std::io::stdout().flush()?;
                let lastname = read_line()?;

                let client_id = moco_client.get_user_id(firstname, lastname).await?;

                config.borrow_mut().moco_user_id = client_id;
                config.borrow_mut().write_config()?;
                println!("🤩 Logged in 🤩")
            }
        },
        cli::Commands::List => {
            let today = Utc::now();
            let monday = today.beginning_of_week();
            let sunday = today.end_of_week();

            let activities = moco_client
                .get_activities(
                    monday.format("%Y-%m-%d").to_string(),
                    sunday.format("%Y-%m-%d").to_string(),
                    None,
                    None,
                )
                .await?;

            for activitie in activities.iter() {
                println!(
                    "{} {}h {} ",
                    activitie.date, activitie.hours, activitie.description
                )
            }
        }
        cli::Commands::New => {
            let (project, task) = promp_task_select(&moco_client).await?;

            println!("Date (YYYY-mm-DD)");
            let date = read_line()?;

            println!("Time in Hours");
            let hours = read_line()?;

            moco_client
                .create_activitie(&CreateActivitie {
                    date,
                    project_id: project.id,
                    task_id: task.id,
                    hours: Some(hours.parse::<f64>()?),
                    ..Default::default()
                })
                .await?;
        }
        cli::Commands::Add => todo!(),
        cli::Commands::Edit => todo!(),
        cli::Commands::Rm => todo!(),
        cli::Commands::Sync {
            system,
            today,
            week,
            month,
        } => match system {
            cli::Sync::Jira => {
                let now = Utc::now();

                let mut from = if today { Some(now) } else { None };
                let mut to = if today { Some(now) } else { None };
                from = if week {
                    Some(now.beginning_of_week())
                } else {
                    from
                };
                to = if week { Some(now.end_of_week()) } else { to };
                from = if month {
                    Some(now.beginning_of_month())
                } else {
                    from
                };
                to = if month { Some(now.end_of_month()) } else { to };

                let from = from.unwrap_or(now);
                let to = to.unwrap_or(now);

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
                            activity.remote_id.parse::<i64>().unwrap_or(0)
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

                for worklog in worklogs {
                    if let Ok(worklog) = &worklog {
                        moco_client.create_activitie(worklog).await?;
                    }

                    println!("{:#?}", &worklog)
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
    let project_index = render_list(
        &projects,
        "Index-Customer-Project-Project ID",
        "Chose your Project:",
        &(|(index, project)| {
            println!(
                "{}-{}-{}-{}",
                index, project.customer.name, project.name, project.id
            )
        }),
    )?;
    let project = &projects[project_index];
    let task_index = render_list(
        &project.tasks,
        "Index-Task-Task ID",
        "Chose your Task:",
        &(|(index, task)| {
            println!("{}-{}-{}", index, task.name, task.id);
        }),
    )?;
    let task = &project.tasks[task_index];
    Ok((project.clone(), task.clone()))
}

fn render_list<T>(
    list: &[T],
    headline: &str,
    promt: &str,
    linenderer: &dyn Fn((usize, &T)),
) -> Result<usize, Box<dyn Error>> {
    loop {
        println!("{}", headline);
        for elem in list.iter().enumerate() {
            linenderer(elem);
        }
        print!("{}", promt);
        std::io::stdout().flush()?;

        let index_input = read_line().map(|x| x.parse::<usize>().ok()).ok().flatten();

        if let Some(index) = index_input {
            if index < list.len() {
                return Ok(index);
            }
        }
        println!("Index Invallid")
    }
}
