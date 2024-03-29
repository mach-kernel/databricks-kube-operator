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
pub struct JobsRunsExport200Response {
    /// The exported content in HTML format (one for every view item).
    #[serde(rename = "views", skip_serializing_if = "Option::is_none")]
    pub views: Option<Vec<crate::models::ViewItem>>,
}

impl JobsRunsExport200Response {
    pub fn new() -> JobsRunsExport200Response {
        JobsRunsExport200Response {
            views: None,
        }
    }
}


