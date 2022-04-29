use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

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
