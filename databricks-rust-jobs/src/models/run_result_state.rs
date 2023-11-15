use schemars::JsonSchema;
/*
 * Jobs API 2.1
 *
 * The Jobs API allows you to create, edit, and delete jobs. You should never hard code secrets or store them in plain text. Use the [Secrets API](https://docs.databricks.com/dev-tools/api/latest/secrets.html) to manage secrets in the [Databricks CLI](https://docs.databricks.com/dev-tools/cli/index.html). Use the [Secrets utility](https://docs.databricks.com/dev-tools/databricks-utils.html#dbutils-secrets) to reference secrets in notebooks and jobs.
 *
 * The version of the OpenAPI document: 2.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RunResultState : * `SUCCESS`: The task completed successfully. * `FAILED`: The task completed with an error. * `TIMEDOUT`: The run was stopped after reaching the timeout. * `CANCELED`: The run was canceled at user request.

/// * `SUCCESS`: The task completed successfully. * `FAILED`: The task completed with an error. * `TIMEDOUT`: The run was stopped after reaching the timeout. * `CANCELED`: The run was canceled at user request.
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RunResultState {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "TIMEDOUT")]
    Timedout,
    #[serde(rename = "CANCELED")]
    Canceled,
    #[serde(rename = "MAXIMUM_CONCURRENT_RUNS_REACHED")]
    MaximumConcurrentRunsReached,

}

impl ToString for RunResultState {
    fn to_string(&self) -> String {
        match self {
            Self::Success => String::from("SUCCESS"),
            Self::Failed => String::from("FAILED"),
            Self::Timedout => String::from("TIMEDOUT"),
            Self::Canceled => String::from("CANCELED"),
            Self::MaximumConcurrentRunsReached => String::from("MAXIMUM_CONCURRENT_RUNS_REACHED"),
        }
    }
}

impl Default for RunResultState {
    fn default() -> RunResultState {
        Self::Success
    }
}




