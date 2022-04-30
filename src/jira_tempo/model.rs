use serde::Deserialize;
use serde::Serialize;

// Employment

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    time_spent_seconds: i64,
    billable_seconds: i64,
    start_date: String,
    start_time: String,
    created_at: String,
    updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    results: Result,
}
