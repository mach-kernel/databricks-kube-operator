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
pub struct SqlDashboardWidgetOutput {
    /// The canonical identifier of the SQL widget.
    #[serde(rename = "widget_id", skip_serializing_if = "Option::is_none")]
    pub widget_id: Option<String>,
    /// The title of the SQL widget.
    #[serde(rename = "widget_title", skip_serializing_if = "Option::is_none")]
    pub widget_title: Option<String>,
    /// The link to find the output results.
    #[serde(rename = "output_link", skip_serializing_if = "Option::is_none")]
    pub output_link: Option<String>,
    /// The execution status of the SQL widget.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<crate::models::SqlOutputError>>,
    /// Time (in epoch milliseconds) when execution of the SQL widget starts.
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// Time (in epoch milliseconds) when execution of the SQL widget ends.
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

impl SqlDashboardWidgetOutput {
    pub fn new() -> SqlDashboardWidgetOutput {
        SqlDashboardWidgetOutput {
            widget_id: None,
            widget_title: None,
            output_link: None,
            status: None,
            error: None,
            start_time: None,
            end_time: None,
        }
    }
}

/// The execution status of the SQL widget.
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "CANCELLED")]
    Cancelled,
}

impl Default for Status {
    fn default() -> Status {
        Self::Pending
    }
}

