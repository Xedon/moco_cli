use config::Config;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{create_dir, write, File},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub api_key: Option<String>,
    pub user_id: Option<i64>,
}

fn get_config_path() -> Option<std::path::PathBuf> {
    dirs::config_dir().map(|dir| dir.join("mococp").join("mococp.json"))
}

pub fn init() -> Result<AppConfig, Box<dyn Error>> {
    let config_file = get_config_path();
    let config_file = match config_file {
        Some(path) => {
            if !path.exists() {
                if !&path.parent().unwrap().exists() {
                    create_dir(&path.parent().unwrap())?;
                }
                File::create(&path)?;
                write(&path, "{}")?;
            }
            path
        }
        None => panic!("Cant find os config directory"),
    };
    Ok(Config::builder()
        .add_source(config::File::from(config_file))
        .build()?
        .try_deserialize::<AppConfig>()?)
}

impl AppConfig {
    pub fn write_config(&self) -> Result<(), Box<dyn Error>> {
        let config_file = get_config_path();
        match config_file {
            Some(file) => {
                let json_string = serde_json::to_string(self)?;
                write(file, json_string)?;
            }
            None => panic!("Cant find os config directory"),
        };
        Ok(())
    }
}
