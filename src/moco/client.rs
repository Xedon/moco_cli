use std::{cell::RefCell, error::Error, sync::Arc};

use reqwest::Client;

use crate::moco::model::{Activitie, CreateActivitie, Employment, Projects};

use crate::config::AppConfig;

pub struct MocoClient {
    client: Client,
    config: Arc<RefCell<AppConfig>>,
}

#[derive(Debug, derive_more::Display)]
enum MocoClientError {
    NotLoggedIn,
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
        match &self.config.borrow().moco_api_key {
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
            None => Err(Box::new(MocoClientError::NotLoggedIn)),
        }
    }

    pub async fn get_activities(
        &self,
        from: String,
        to: String,
        task_id: Option<String>,
        term: Option<String>,
    ) -> Result<Vec<Activitie>, Box<dyn Error>> {
        let mut parameter = vec![
            ("from", from),
            ("to", to),
            (
                "user_id",
                format!("{}", &self.config.borrow().moco_user_id.unwrap()),
            ),
        ];

        if let Some(x) = task_id {
            parameter.push(("task_id", x))
        }
        if let Some(x) = term {
            parameter.push(("term", x))
        }

        match &self.config.borrow().moco_api_key {
            Some(api_key) => Ok(self
                .client
                .get("https://mayflower.mocoapp.com/api/v1/activities")
                .query(&parameter)
                .header("Authorization", format!("Token token={}", api_key))
                .send()
                .await?
                .json::<Vec<Activitie>>()
                .await?),
            None => Err(Box::new(MocoClientError::NotLoggedIn)),
        }
    }

    pub async fn create_activitie(&self, payload: &CreateActivitie) -> Result<(), Box<dyn Error>> {
        match &self.config.borrow().moco_api_key {
            Some(api_key) => {
                self.client
                    .post("https://mayflower.mocoapp.com/api/v1/activities")
                    .header("Authorization", format!("Token token={}", api_key))
                    .json(payload)
                    .send()
                    .await?
                    .json::<Vec<Activitie>>()
                    .await?;
                Ok(())
            }
            None => Err(Box::new(MocoClientError::NotLoggedIn)),
        }
    }

    pub async fn get_assigned_projects(&self) -> Result<Projects, Box<dyn Error>> {
        match &self.config.borrow().moco_api_key {
            Some(api_key) => Ok(self
                .client
                .get("https://mayflower.mocoapp.com/api/v1/projects/assigned")
                .header("Authorization", format!("Token token={}", api_key))
                .send()
                .await?
                .json::<Projects>()
                .await?),
            None => Err(Box::new(MocoClientError::NotLoggedIn)),
        }
    }
}
