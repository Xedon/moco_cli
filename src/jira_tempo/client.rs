use std::{cell::RefCell, error::Error, sync::Arc};

use reqwest::Client;

use crate::config::AppConfig;

pub struct JiraTempoClient {
    client: Client,
    config: Arc<RefCell<AppConfig>>,
}

#[derive(Debug, derive_more::Display)]
enum JiraTempoClientError {
    NotLoggedIn,
}
impl Error for JiraTempoClientError {}

impl JiraTempoClient {
    pub fn new(app_config: &Arc<RefCell<AppConfig>>) -> Self {
        JiraTempoClient {
            client: Client::new(),
            config: app_config.clone(),
        }
    }
}
