use std::{cell::RefCell, error::Error, sync::Arc};

use reqwest::Client;

use crate::moco::model::{
    Activitie,
    CreateActivitie,
    EditActivitie,
    Employment,
    Projects
};

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
        let config = &self.config.borrow();
        match (config.moco_api_key.as_ref(), config.moco_company.as_ref()) {
            (Some(api_key), Some(company)) => {
                let employments = self
                    .client
                    .get(format!(
                        "https://{company}.mocoapp.com/api/v1/users/employments"
                    ))
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
            (_, _) => Err(Box::new(MocoClientError::NotLoggedIn)),
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

        let config = &self.config.borrow();
        match (config.moco_api_key.as_ref(), config.moco_company.as_ref()) {
            (Some(api_key), Some(company)) => Ok(self
                .client
                .get(format!("https://{company}.mocoapp.com/api/v1/activities"))
                .query(&parameter)
                .header("Authorization", format!("Token token={}", api_key))
                .send()
                .await?
                .json::<Vec<Activitie>>()
                .await?),
            (_, _) => Err(Box::new(MocoClientError::NotLoggedIn)),
        }
    }

    pub async fn create_activitie(&self, payload: &CreateActivitie) -> Result<(), Box<dyn Error>> {
        let config = &self.config.borrow();
        match (config.moco_api_key.as_ref(), config.moco_company.as_ref()) {
            (Some(api_key), Some(company)) => {
                self.client
                    .post(format!("https://{company}.mocoapp.com/api/v1/activities"))
                    .header("Authorization", format!("Token token={}", api_key))
                    .json(payload)
                    .send()
                    .await?;
                Ok(())
            }
            (_, _) => Err(Box::new(MocoClientError::NotLoggedIn)),
        }
    }

    pub async fn edit_activitie(&self, payload: &EditActivitie) -> Result<(), Box<dyn Error>> {
        let config = &self.config.borrow();
        match (config.moco_api_key.as_ref(), config.moco_company.as_ref()) {
            (Some(api_key), Some(company)) => {
                self.client
                    .put(format!("https://{company}.mocoapp.com/api/v1/activities/{}", payload.activity_id))
                    .header("Authorization", format!("Token token={}", api_key))
                    .json(payload)
                    .send()
                    .await?;
                Ok(())
            }
            (_, _) => Err(Box::new(MocoClientError::NotLoggedIn)),
        }
    }

    pub async fn get_assigned_projects(&self) -> Result<Projects, Box<dyn Error>> {
        let config = &self.config.borrow();
        match (config.moco_api_key.as_ref(), config.moco_company.as_ref()) {
            (Some(api_key), Some(company)) => Ok(self
                .client
                .get(format!(
                    "https://{company}.mocoapp.com/api/v1/projects/assigned"
                ))
                .header("Authorization", format!("Token token={}", api_key))
                .send()
                .await?
                .json::<Projects>()
                .await?),
            (_, _) => Err(Box::new(MocoClientError::NotLoggedIn)),
        }
    }
}
