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
pub struct SqlTask {
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<Box<crate::models::SqlTaskQuery>>,
    #[serde(rename = "dashboard", skip_serializing_if = "Option::is_none")]
    pub dashboard: Option<Box<crate::models::SqlTaskDashboard>>,
    #[serde(rename = "alert", skip_serializing_if = "Option::is_none")]
    pub alert: Option<Box<crate::models::SqlTaskAlert>>,
    /// Parameters to be used for each run of this job. The SQL alert task does not support custom parameters.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, serde_json::Value>>,
    /// The canonical identifier of the SQL warehouse. Only serverless and pro SQL warehouses are supported.
    #[serde(rename = "warehouse_id")]
    pub warehouse_id: String,
}

impl SqlTask {
    pub fn new(warehouse_id: String) -> SqlTask {
        SqlTask {
            query: None,
            dashboard: None,
            alert: None,
            parameters: None,
            warehouse_id,
        }
    }
}


