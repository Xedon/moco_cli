use std::{
    cell::RefCell,
    error::Error,
    rc::Rc,
    sync::{Arc, RwLock},
};

use reqwest::{Client, Response};

use crate::{config::AppConfig, moco_model::Employment};
pub struct MocoClient {
    client: Client,
    config: Arc<RefCell<AppConfig>>,
}

#[derive(Debug, derive_more::Display)]
enum MocoClientError {
    ConfigError,
}
impl Error for MocoClientError {}

impl MocoClient {
    pub fn new(app_config: &Arc<RefCell<AppConfig>>) -> Self {
        MocoClient {
            client: Client::new(),
            config: app_config.clone(),
        }
    }

    pub async fn get_user_id(
        &self,
        firstname: String,
        lastname: String,
    ) -> Result<Option<i64>, Box<dyn Error>> {
        match &self.config.borrow().api_key {
            Some(api_key) => {
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
