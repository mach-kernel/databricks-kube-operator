use schemars::JsonSchema;
/*
 * Databricks Accounts and Workspace REST API on ALL
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(JsonSchema, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobsRunOutput {
    #[serde(rename = "condition_task", skip_serializing_if = "Option::is_none")]
    pub condition_task: Option<serde_json::Value>,
    /// An error message indicating why a task failed or why output is not available. The message is unstructured, and its exact format is subject to change.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The output of a notebook task, if available. A notebook task that terminates (either successfully or with a failure) without calling `Dbutilsnotebook.exit()` is considered to have an empty output. This field is set but its result value is empty. <Databricks> restricts this API to return the first 5 MB of the output. To return a larger result, use the [ClusterLogConf](/dev-tools/api/latest/clusters.html#clusterlogconf) field to configure log storage for the job cluster. 
    #[serde(rename = "notebook_output", skip_serializing_if = "Option::is_none")]
    pub notebook_output: Option<Box<crate::models::JobsNotebookOutput>>,
    /// The output of a dbt task, if available.
    #[serde(rename = "dbt_output", skip_serializing_if = "Option::is_none")]
    pub dbt_output: Option<Box<crate::models::JobsDbtOutput>>,
    /// The output of a SQL task, if available.
    #[serde(rename = "sql_output", skip_serializing_if = "Option::is_none")]
    pub sql_output: Option<Box<crate::models::JobsSqlOutput>>,
    /// The output of a run job task, if available
    #[serde(rename = "run_job_output", skip_serializing_if = "Option::is_none")]
    pub run_job_output: Option<Box<crate::models::JobsRunJobOutput>>,
    /// The output from tasks that write to standard streams (stdout/stderr) such as spark_jar_task, spark_python_task, python_wheel_task.  It's not supported for the notebook_task, pipeline_task or spark_submit_task.  Databricks restricts this API to return the last 5 MB of these logs. 
    #[serde(rename = "logs", skip_serializing_if = "Option::is_none")]
    pub logs: Option<String>,
    /// All details of the run except for its output.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<crate::models::JobsRun>>,
    /// If there was an error executing the run, this field contains any available stack traces.
    #[serde(rename = "error_trace", skip_serializing_if = "Option::is_none")]
    pub error_trace: Option<String>,
    /// Whether the logs are truncated.
    #[serde(rename = "logs_truncated", skip_serializing_if = "Option::is_none")]
    pub logs_truncated: Option<bool>,
}

impl JobsRunOutput {
    pub fn new() -> JobsRunOutput {
        JobsRunOutput {
            condition_task: None,
            error: None,
            notebook_output: None,
            dbt_output: None,
            sql_output: None,
            run_job_output: None,
            logs: None,
            metadata: None,
            error_trace: None,
            logs_truncated: None,
        }
    }
}


