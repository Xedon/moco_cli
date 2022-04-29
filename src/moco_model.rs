use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

// Employment

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Employment {
    pub id: i64,
    #[serde(rename = "weekly_target_hours")]
    pub weekly_target_hours: f64,
    pub pattern: Pattern,
    pub from: String,
    pub to: Value,
    pub user: User,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pattern {
    pub am: Vec<f64>,
    pub pm: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub firstname: String,
    pub lastname: String,
}

// Activities

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activitie {
    pub id: i64,
    pub date: String,
    pub hours: f64,
    pub seconds: i64,
    pub description: String,
    pub billed: bool,
    pub billable: bool,
    pub tag: String,
    #[serde(rename = "remote_service")]
    pub remote_service: String,
    #[serde(rename = "remote_id")]
    pub remote_id: Value,
    #[serde(rename = "remote_url")]
    pub remote_url: Value,
    pub project: Project,
    pub task: Task,
    pub customer: Customer,
    pub user: User,
    #[serde(rename = "timer_started_at")]
    pub timer_started_at: Value,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "hourly_rate")]
    pub hourly_rate: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub billable: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: i64,
    pub name: String,
    pub billable: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub id: i64,
    pub name: String,
}
