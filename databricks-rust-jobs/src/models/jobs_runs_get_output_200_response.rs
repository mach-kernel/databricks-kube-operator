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
pub struct JobsRunsGetOutput200Response {
    #[serde(rename = "notebook_output", skip_serializing_if = "Option::is_none")]
    pub notebook_output: Option<Box<crate::models::NotebookOutput>>,
    #[serde(rename = "sql_output", skip_serializing_if = "Option::is_none")]
    pub sql_output: Option<Box<crate::models::SqlOutput>>,
    #[serde(rename = "dbt_output", skip_serializing_if = "Option::is_none")]
    pub dbt_output: Option<Box<crate::models::DbtOutput>>,
    /// The output from tasks that write to standard streams (stdout/stderr) such as [SparkJarTask](https://docs.databricks.com/dev-tools/api/latest/jobs.html#/components/schemas/SparkJarTask), [SparkPythonTask](https://docs.databricks.com/dev-tools/api/latest/jobs.html#/components/schemas/SparkPythonTask, [PythonWheelTask](https://docs.databricks.com/dev-tools/api/latest/jobs.html#/components/schemas/PythonWheelTask. It's not supported for the [NotebookTask](https://docs.databricks.com/dev-tools/api/latest/jobs.html#/components/schemas/NotebookTask, [PipelineTask](https://docs.databricks.com/dev-tools/api/latest/jobs.html#/components/schemas/PipelineTask, or [SparkSubmitTask](https://docs.databricks.com/dev-tools/api/latest/jobs.html#/components/schemas/SparkSubmitTask. Databricks restricts this API to return the last 5 MB of these logs.
    #[serde(rename = "logs", skip_serializing_if = "Option::is_none")]
    pub logs: Option<String>,
    /// Whether the logs are truncated.
    #[serde(rename = "logs_truncated", skip_serializing_if = "Option::is_none")]
    pub logs_truncated: Option<bool>,
    /// An error message indicating why a task failed or why output is not available. The message is unstructured, and its exact format is subject to change.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// If there was an error executing the run, this field contains any available stack traces.
    #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
    pub error_trace: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::Run>>,
}

impl JobsRunsGetOutput200Response {
    pub fn new() -> JobsRunsGetOutput200Response {
        JobsRunsGetOutput200Response {
            notebook_output: None,
            sql_output: None,
            dbt_output: None,
            logs: None,
            logs_truncated: None,
            error: None,
            error_trace: None,
            metadata: None,
        }
    }
}


