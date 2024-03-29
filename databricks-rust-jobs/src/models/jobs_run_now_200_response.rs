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




#[derive(JsonSchema, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobsRunNow200Response {
    /// The globally unique ID of the newly triggered run.
    #[serde(rename = "run_id", skip_serializing_if = "Option::is_none")]
    pub run_id: Option<i64>,
    /// A unique identifier for this job run. This is set to the same value as `run_id`.
    #[serde(rename = "number_in_job", skip_serializing_if = "Option::is_none")]
    pub number_in_job: Option<i64>,
}

impl JobsRunNow200Response {
    pub fn new() -> JobsRunNow200Response {
        JobsRunNow200Response {
            run_id: None,
            number_in_job: None,
        }
    }
}


