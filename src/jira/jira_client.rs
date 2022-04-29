use std::error::Error;

use reqwest::{Client, Response};

use crate::config::AppConfig;
pub struct JiraClient<'a> {
    client: Client,
    config: &'a AppConfig,
}

#[derive(Debug, derive_more::Display)]
enum JiraClientError {
    ConfigError,
}
impl Error for JiraClientError {}

impl<'a> JiraClient<'a> {
    pub fn new(app_config: &'a AppConfig) -> Self {
        JiraClient {
            client: Client::new(),
            config: app_config,
        }
    }

    pub async fn test_api_key(&self) -> Result<(), Box<dyn Error>> {
        match &self.config.api_key {
            Some(api_key) => {
                self.client
                    .get("")
                    .header("Authorization", format!("Token token={}", api_key))
                    .send()
                    .await?;
                Ok(())
            }
            None => Err(Box::new(JiraClientError::ConfigError)),
        }
    }
}
