pub struct Result {
    timeSpentSeconds: i64,
    billableSeconds: i64,
    startDate: String,
    startTime: String,
    createdAt: String,
    updatedAt: String,
}

pub struct Response {
    results: Result,
}
