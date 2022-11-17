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
pub struct SqlOutput {
    #[serde(rename = "query_output", skip_serializing_if = "Option::is_none")]
    pub query_output: Option<Box<crate::models::SqlQueryOutput>>,
    #[serde(rename = "dashboard_output", skip_serializing_if = "Option::is_none")]
    pub dashboard_output: Option<Box<crate::models::SqlDashboardOutput>>,
    #[serde(rename = "alert_output", skip_serializing_if = "Option::is_none")]
    pub alert_output: Option<Box<crate::models::SqlAlertOutput>>,
}

impl SqlOutput {
    pub fn new() -> SqlOutput {
        SqlOutput {
            query_output: None,
            dashboard_output: None,
            alert_output: None,
        }
    }
}