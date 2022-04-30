use std::{cell::RefCell, error::Error, io::Write, sync::Arc};

use crate::moco::client::MocoClient;
use chrono::Utc;

use crate::moco::model::CreateActivitie;

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
    env_logger::init();
    let args = cli::init();
    let config = Arc::new(RefCell::new(config::init()?));
    let moco_client = MocoClient::new(&config);

    match args.command {
        cli::Commands::Login { system } => match system {
            cli::Login::Jira => todo!(),
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
                println!("🤩 Logged In 🤩")
            }
        },
        cli::Commands::List => {
            use now::DateTimeNow;
            let today = Utc::now();
            let monday = today.beginning_of_week();
            let sunday = today.end_of_week();

            let activities = moco_client
                .get_activities(
                    monday.format("%Y-%m-%d").to_string(),
                    sunday.format("%Y-%m-%d").to_string(),
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

            println!("Date (YYYY-mm-DD)");
            let date = read_line()?;

            println!("Time in Hours");
            let hours = read_line()?;

            moco_client
                .create_activitie(&CreateActivitie {
                    date,
                    project_id: project.id,
                    task_id: task.id,
                    hours: hours.parse::<f64>()?,
                    ..Default::default()
                })
                .await?;
        }
        cli::Commands::Add => todo!(),
        cli::Commands::Edit => todo!(),
        cli::Commands::Rm => todo!(),
    }

    Ok(())
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
