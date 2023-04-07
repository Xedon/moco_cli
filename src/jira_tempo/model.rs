use serde::Deserialize;
use serde::Serialize;

// Employment

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub time_spent_seconds: i64,
    pub billable_seconds: i64,
    pub start_date: String,
    pub start_time: String,
    pub created_at: String,
    pub updated_at: String,
    pub issue: Issue,
    pub description: String,
    pub jira_worklog_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Issue {
    pub key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
    pub results: Vec<Result>,
}
