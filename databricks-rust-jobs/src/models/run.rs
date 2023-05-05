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
pub struct Run {
    /// The canonical identifier of the job that contains this run.
    #[serde(rename = "job_id", skip_serializing_if = "Option::is_none")]
    pub job_id: Option<i64>,
    /// The canonical identifier of the run. This ID is unique across all runs of all jobs.
    #[serde(rename = "run_id", skip_serializing_if = "Option::is_none")]
    pub run_id: Option<i64>,
    /// A unique identifier for this job run. This is set to the same value as `run_id`.
    #[serde(rename = "number_in_job", skip_serializing_if = "Option::is_none")]
    pub number_in_job: Option<i64>,
    /// The creator user name. This field won’t be included in the response if the user has already been deleted.
    #[serde(rename = "creator_user_name", skip_serializing_if = "Option::is_none")]
    pub creator_user_name: Option<String>,
    /// If this run is a retry of a prior run attempt, this field contains the run_id of the original attempt; otherwise, it is the same as the run_id.
    #[serde(
        rename = "original_attempt_run_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub original_attempt_run_id: Option<i64>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<crate::models::RunState>>,
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Box<crate::models::CronSchedule>>,
    #[serde(rename = "continuous", skip_serializing_if = "Option::is_none")]
    pub continuous: Option<Box<crate::models::Continuous>>,
    /// The list of tasks performed by the run. Each task has its own `run_id` which you can use to call `JobsGetOutput` to retrieve the run resutls.
    #[serde(rename = "tasks", skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<crate::models::RunTask>>,
    /// A list of job cluster specifications that can be shared and reused by tasks of this job. Libraries cannot be declared in a shared job cluster. You must declare dependent libraries in task settings.
    #[serde(rename = "job_clusters", skip_serializing_if = "Option::is_none")]
    pub job_clusters: Option<Vec<crate::models::JobCluster>>,
    #[serde(rename = "cluster_spec", skip_serializing_if = "Option::is_none")]
    pub cluster_spec: Option<Box<crate::models::ClusterSpec>>,
    #[serde(rename = "cluster_instance", skip_serializing_if = "Option::is_none")]
    pub cluster_instance: Option<Box<crate::models::ClusterInstance>>,
    #[serde(rename = "git_source", skip_serializing_if = "Option::is_none")]
    pub git_source: Option<Box<crate::models::GitSource>>,
    #[serde(
        rename = "overriding_parameters",
        skip_serializing_if = "Option::is_none"
    )]
    pub overriding_parameters: Option<Box<crate::models::RunParameters>>,
    /// The time at which this run was started in epoch milliseconds (milliseconds since 1/1/1970 UTC). This may not be the time when the job task starts executing, for example, if the job is scheduled to run on a new cluster, this is the time the cluster creation call is issued.
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// The time in milliseconds it took to set up the cluster. For runs that run on new clusters this is the cluster creation time, for runs that run on existing clusters this time should be very short. The duration of a task run is the sum of the `setup_duration`, `execution_duration`, and the `cleanup_duration`. The `setup_duration` field is set to 0 for multitask job runs. The total duration of a multitask job run is the value of the `run_duration` field.
    #[serde(rename = "setup_duration", skip_serializing_if = "Option::is_none")]
    pub setup_duration: Option<i64>,
    /// The time in milliseconds it took to execute the commands in the JAR or notebook until they  completed, failed, timed out, were cancelled, or encountered an unexpected error. The duration of a task run is the sum of the `setup_duration`, `execution_duration`, and the  `cleanup_duration`. The `execution_duration` field is set to 0 for multitask job runs. The total  duration of a multitask job run is the value of the `run_duration` field.
    #[serde(rename = "execution_duration", skip_serializing_if = "Option::is_none")]
    pub execution_duration: Option<i64>,
    /// The time in milliseconds it took to terminate the cluster and clean up any associated artifacts. The duration of a task run is the sum of the `setup_duration`, `execution_duration`, and the `cleanup_duration`. The `cleanup_duration` field is set to 0 for multitask job runs. The total duration of a multitask job run is the value of the `run_duration` field.
    #[serde(rename = "cleanup_duration", skip_serializing_if = "Option::is_none")]
    pub cleanup_duration: Option<i64>,
    /// The time at which this run ended in epoch milliseconds (milliseconds since 1/1/1970 UTC). This field is set to 0 if the job is still running.
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// The time in milliseconds it took the job run and all of its repairs to finish. This field is only set for multitask job runs and not task runs. The duration of a task run is the sum of the `setup_duration`, `execution_duration`, and the `cleanup_duration`.
    #[serde(rename = "run_duration", skip_serializing_if = "Option::is_none")]
    pub run_duration: Option<i32>,
    #[serde(rename = "trigger", skip_serializing_if = "Option::is_none")]
    pub trigger: Option<crate::models::TriggerType>,
    /// An optional name for the run. The maximum allowed length is 4096 bytes in UTF-8 encoding.
    #[serde(rename = "run_name", skip_serializing_if = "Option::is_none")]
    pub run_name: Option<String>,
    /// The URL to the detail page of the run.
    #[serde(rename = "run_page_url", skip_serializing_if = "Option::is_none")]
    pub run_page_url: Option<String>,
    #[serde(rename = "run_type", skip_serializing_if = "Option::is_none")]
    pub run_type: Option<crate::models::RunType>,
    /// The sequence number of this run attempt for a triggered job run. The initial attempt of a run has an attempt_number of 0\\. If the initial run attempt fails, and the job has a retry policy (`max_retries` \\> 0), subsequent runs are created with an `original_attempt_run_id` of the original attempt’s ID and an incrementing `attempt_number`. Runs are retried only until they succeed, and the maximum `attempt_number` is the same as the `max_retries` value for the job.
    #[serde(rename = "attempt_number", skip_serializing_if = "Option::is_none")]
    pub attempt_number: Option<i32>,
}

impl Run {
    pub fn new() -> Run {
        Run {
            job_id: None,
            run_id: None,
            number_in_job: None,
            creator_user_name: None,
            original_attempt_run_id: None,
            state: None,
            schedule: None,
            continuous: None,
            tasks: None,
            job_clusters: None,
            cluster_spec: None,
            cluster_instance: None,
            git_source: None,
            overriding_parameters: None,
            start_time: None,
            setup_duration: None,
            execution_duration: None,
            cleanup_duration: None,
            end_time: None,
            run_duration: None,
            trigger: None,
            run_name: None,
            run_page_url: None,
            run_type: None,
            attempt_number: None,
        }
    }
}
