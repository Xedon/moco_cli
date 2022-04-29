use std::{
    error::Error,
    fs::{create_dir, File},
};

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
    let mut config = config::init()?;
    let moco_client = MocoClient::new();

    match args.command {
        cli::Commands::Login => {
            let mut firstname = String::new();
            let mut lastname = String::new();

            let mut api_key = String::new();
            println!("Enter your personal api key");
            std::io::stdin().read_line(&mut api_key)?;
            api_key.remove(api_key.len() - 1);
            config.api_key = Some(api_key);

            println!("Enter firstname");
            std::io::stdin().read_line(&mut firstname)?;
            firstname.remove(firstname.len() - 1);

            println!("Enter lastname");
            std::io::stdin().read_line(&mut lastname)?;
            lastname.remove(lastname.len() - 1);

            let client_id = moco_client
                .get_user_id(&config, firstname, lastname)
                .await?;

            println!("{:?}", client_id);
            config.user_id = client_id;
            config.write_config()?;
            println!("Config written!")
        }
        cli::Commands::List => todo!(),
        cli::Commands::New => todo!(),
        cli::Commands::Add => todo!(),
        cli::Commands::Edit => todo!(),
        cli::Commands::Rm => todo!(),
    }

    Ok(())
}
