use std::error::Error;

use reqwest::Client;

use crate::{config::AppConfig, moco_model::Employment};
pub struct MocoClient {
    client: Client,
}

#[derive(Debug, derive_more::Display)]
enum MocoClientError {
    ConfigError,
}
impl Error for MocoClientError {}

impl MocoClient {
    pub fn new() -> Self {
        MocoClient {
            client: Client::new(),
        }
    }

    pub async fn get_user_id(
        &self,
        app_config: &AppConfig,
        firstname: String,
        lastname: String,
    ) -> Result<Option<i64>, Box<dyn Error>> {
        match &app_config.api_key {
            Some(api_key) => {
                println!("{}", api_key);
                let employments = self
                    .client
                    .get("https://mayflower.mocoapp.com/api/v1/users/employments")
                    .header("Authorization", format!("Token token={}", api_key))
                    .send()
                    .await?
                    .json::<Vec<Employment>>()
                    .await?;
                Ok(employments
                    .iter()
                    .find(|employment| {
                        employment.user.firstname == firstname
                            && employment.user.lastname == lastname
                    })
                    .map(|employment| employment.user.id))
            }
            None => Err(Box::new(MocoClientError::ConfigError)),
        }
    }
}
