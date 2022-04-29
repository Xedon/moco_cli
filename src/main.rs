use std::{
    cell::RefCell,
    error::Error,
    fs::{create_dir, File},
    sync::Arc,
};

use chrono::{Date, DateTime, Datelike, Duration, Local, Utc};
use moco_client::MocoClient;

mod cli;
mod config;
mod jira;
mod moco_client;
mod moco_model;
mod tempo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let args = cli::init();
    let config = Arc::new(RefCell::new(config::init()?));
    let moco_client = MocoClient::new(&config);

    match args.command {
        cli::Commands::Login => {
            let mut firstname = String::new();
            let mut lastname = String::new();

            let mut api_key = String::new();
            println!("Enter your personal api key");
            std::io::stdin().read_line(&mut api_key)?;
            api_key.remove(api_key.len() - 1);
            config.borrow_mut().api_key = Some(api_key);

            println!("Enter firstname");
            std::io::stdin().read_line(&mut firstname)?;
            firstname.remove(firstname.len() - 1);

            println!("Enter lastname");
            std::io::stdin().read_line(&mut lastname)?;
            lastname.remove(lastname.len() - 1);

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
        cli::Commands::New => todo!(),
        cli::Commands::Add => todo!(),
        cli::Commands::Edit => todo!(),
        cli::Commands::Rm => todo!(),
    }

    Ok(())
}
