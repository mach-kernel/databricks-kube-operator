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
pub struct JobsRunsList200Response {
    /// A list of runs, from most recently started to least.
    #[serde(rename = "runs", skip_serializing_if = "Option::is_none")]
    pub runs: Option<Vec<crate::models::Run>>,
    /// If true, additional runs matching the provided filter are available for listing.
    #[serde(rename = "has_more", skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl JobsRunsList200Response {
    pub fn new() -> JobsRunsList200Response {
        JobsRunsList200Response {
            runs: None,
            has_more: None,
        }
    }
}


