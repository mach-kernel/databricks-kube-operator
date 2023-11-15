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




#[derive(JsonSchema, Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobsUpdateRequest {
    /// The canonical identifier of the job to update. This field is required.
    #[serde(rename = "job_id")]
    pub job_id: i64,
    #[serde(rename = "new_settings", skip_serializing_if = "Option::is_none")]
    pub new_settings: Option<Box<crate::models::JobSettings>>,
    /// Remove top-level fields in the job settings. Removing nested fields is not supported. This field is optional.
    #[serde(rename = "fields_to_remove", skip_serializing_if = "Option::is_none")]
    pub fields_to_remove: Option<Vec<String>>,
}

impl JobsUpdateRequest {
    pub fn new(job_id: i64) -> JobsUpdateRequest {
        JobsUpdateRequest {
            job_id,
            new_settings: None,
            fields_to_remove: None,
        }
    }
}


