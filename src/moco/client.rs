use std::{error::Error, sync::Arc};

use reqwest::Client;
use tokio::sync::RwLock;

use crate::moco::model::{
    Activity, ControlActivityTimer, CreateActivity, DeleteActivity, EditActivity, Employment,
    GetActivity, Projects,
};

use crate::config::AppConfig;

pub struct MocoClient {
    client: Client,
    config: Arc<RwLock<AppConfig>>,
}

#[derive(Debug, derive_more::Display)]
enum MocoClientError {
    NotLoggedIn,
}
impl Error for MocoClientError {}

impl MocoClient {
    pub fn new(app_config: &Arc<RwLock<AppConfig>>) -> Self {
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
        let config = self.config.read().await;
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
    ) -> Result<Vec<Activity>, Box<dyn Error>> {
        let mut parameter = vec![
            ("from", from),
            ("to", to),
            (
                "user_id",
                format!("{}", &self.config.read().await.moco_user_id.unwrap()),
            ),
        ];

        if let Some(x) = task_id {
            parameter.push(("task_id", x))
        }
        if let Some(x) = term {
            parameter.push(("term", x))
        }

        let config = &self.config.read().await;
        match (config.moco_api_key.as_ref(), config.moco_company.as_ref()) {
            (Some(api_key), Some(company)) => Ok(self
                .client
                .get(format!("https://{company}.mocoapp.com/api/v1/activities"))
                .query(&parameter)
                .header("Authorization", format!("Token token={}", api_key))
                .send()
                .await?
                .json::<Vec<Activity>>()
                .await?),
            (_, _) => Err(Box::new(MocoClientError::NotLoggedIn)),
        }
    }

    pub async fn get_activity(&self, payload: &GetActivity) -> Result<Activity, Box<dyn Error>> {
        let config = &self.config.read().await;
        match (config.moco_api_key.as_ref(), config.moco_company.as_ref()) {
            (Some(api_key), Some(company)) => Ok(self
                .client
                .get(format!(
                    "https://{company}.mocoapp.com/api/v1/activities/{}",
                    payload.activity_id
                ))
                .header("Authorization", format!("Token token={}", api_key))
                .send()
                .await?
                .json::<Activity>()
                .await?),
            (_, _) => Err(Box::new(MocoClientError::NotLoggedIn)),
        }
    }

    pub async fn create_activity(&self, payload: &CreateActivity) -> Result<(), Box<dyn Error>> {
        let config = &self.config.read().await;
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

    pub async fn edit_activity(&self, payload: &EditActivity) -> Result<(), Box<dyn Error>> {
        let config = &self.config.read().await;
        match (config.moco_api_key.as_ref(), config.moco_company.as_ref()) {
            (Some(api_key), Some(company)) => {
                self.client
                    .put(format!(
                        "https://{company}.mocoapp.com/api/v1/activities/{}",
                        payload.activity_id
                    ))
                    .header("Authorization", format!("Token token={}", api_key))
                    .json(payload)
                    .send()
                    .await?;
                Ok(())
            }
            (_, _) => Err(Box::new(MocoClientError::NotLoggedIn)),
        }
    }

    pub async fn delete_activity(&self, payload: &DeleteActivity) -> Result<(), Box<dyn Error>> {
        let config = &self.config.read().await;
        match (config.moco_api_key.as_ref(), config.moco_company.as_ref()) {
            (Some(api_key), Some(company)) => {
                self.client
                    .delete(format!(
                        "https://{company}.mocoapp.com/api/v1/activities/{}",
                        payload.activity_id
                    ))
                    .header("Authorization", format!("Token token={}", api_key))
                    .send()
                    .await?;
                Ok(())
            }
            (_, _) => Err(Box::new(MocoClientError::NotLoggedIn)),
        }
    }

    pub async fn control_activity_timer(
        &self,
        payload: &ControlActivityTimer,
    ) -> Result<(), Box<dyn Error>> {
        let config = &self.config.read().await;
        match (config.moco_api_key.as_ref(), config.moco_company.as_ref()) {
            (Some(api_key), Some(company)) => {
                self.client
                    .patch(format!(
                        "https://{company}.mocoapp.com/api/v1/activities/{}/{}_timer",
                        payload.activity_id, payload.control
                    ))
                    .header("Authorization", format!("Token token={}", api_key))
                    .send()
                    .await?;
                Ok(())
            }
            (_, _) => Err(Box::new(MocoClientError::NotLoggedIn)),
        }
    }

    pub async fn get_assigned_projects(&self) -> Result<Projects, Box<dyn Error>> {
        let config = &self.config.read().await;
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
