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




#[derive(JsonSchema, Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AutoScale {
    /// The minimum number of workers to which the cluster can scale down when underutilized. It is also the initial number of workers the cluster has after creation.
    #[serde(rename = "min_workers", skip_serializing_if = "Option::is_none")]
    pub min_workers: Option<i32>,
    /// The maximum number of workers to which the cluster can scale up when overloaded. max_workers must be strictly greater than min_workers.
    #[serde(rename = "max_workers", skip_serializing_if = "Option::is_none")]
    pub max_workers: Option<i32>,
}

impl AutoScale {
    pub fn new() -> AutoScale {
        AutoScale {
            min_workers: None,
            max_workers: None,
        }
    }
}


