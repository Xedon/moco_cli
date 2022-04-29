use std::{
    cell::RefCell,
    error::Error,
    fs::{create_dir, File},
    sync::Arc,
};

use chrono::{Date, DateTime, Datelike, Duration, Local, Utc};
use moco_client::MocoClient;

use crate::moco_model::CreateActivitie;

mod cli;
mod config;
mod jira;
mod moco_client;
mod moco_model;
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
        cli::Commands::Login => {
            println!("Enter your personal api key");
            let api_key = read_line()?;
            config.borrow_mut().api_key = Some(api_key);

            println!("Enter firstname");
            let firstname = read_line()?;

            println!("Enter lastname");
            let lastname = read_line()?;

            let client_id = moco_client.get_user_id(firstname, lastname).await?;

            println!("{:?}", client_id);
            config.borrow_mut().user_id = client_id;
            config.borrow_mut().write_config()?;
            println!("Config written!")
        }
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
            println!("Chose your Project:");

            for (index, project) in projects.iter().enumerate() {
                println!("{} {} {}", index, project.customer.name, project.name)
            }

            let project_index = read_line()?;
            let project = &projects[project_index.parse::<usize>()?];

            println!("Chose your Task:");

            for (index, task) in project.tasks.iter().enumerate() {
                println!("{} {}", index, task.name);
            }

            let task_index = read_line()?;
            let task = &project.tasks[task_index.parse::<usize>()?];

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
