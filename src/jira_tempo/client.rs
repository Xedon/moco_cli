use std::{error::Error, sync::Arc};

use reqwest::Client;
use tokio::sync::RwLock;

use crate::config::AppConfig;

use super::model::Response;

const TEMPO_URL: &str = "https://api.tempo.io/core/3";

pub struct JiraTempoClient {
    client: Client,
    config: Arc<RwLock<AppConfig>>,
}

#[derive(Debug, derive_more::Display)]
enum JiraTempoClientError {
    NotLoggedIn,
}
impl Error for JiraTempoClientError {}

impl JiraTempoClient {
    pub fn new(app_config: &Arc<RwLock<AppConfig>>) -> Self {
        JiraTempoClient {
            client: Client::new(),
            config: app_config.clone(),
        }
    }

    pub async fn test_login(&self) -> Result<(), Box<dyn Error>> {
        match &self.config.read().await.jira_tempo_api_key {
            Some(token) => {
                self.client
                    .get(format!("{TEMPO_URL}/globalconfiguration"))
                    .bearer_auth(token)
                    .send()
                    .await?;
                Ok(())
            }
            None => Err(Box::new(JiraTempoClientError::NotLoggedIn)),
        }
    }

    pub async fn get_worklogs(&self, from: String, to: String) -> Result<Response, Box<dyn Error>> {
        let parameter = vec![("from", from), ("to", to), ("limit", "1000".to_string())];

        match &self.config.read().await.jira_tempo_api_key {
            Some(token) => Ok(self
                .client
                .get(format!("{TEMPO_URL}/worklogs"))
                .query(&parameter)
                .bearer_auth(token)
                .send()
                .await?
                .json::<Response>()
                .await?),
            None => Err(Box::new(JiraTempoClientError::NotLoggedIn)),
        }
    }
}
