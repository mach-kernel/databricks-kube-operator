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
pub struct JobTaskSettings {
    /// A unique name for the task. This field is used to refer to this task from other tasks. This field is required and must be unique within its parent job. On Update or Reset, this field is used to reference the tasks to be updated or reset. The maximum length is 100 characters.
    #[serde(rename = "task_key")]
    pub task_key: String,
    /// An optional description for this task. The maximum length is 4096 bytes.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An optional array of objects specifying the dependency graph of the task. All tasks specified in this field must complete successfully before executing this task. The key is `task_key`, and the value is the name assigned to the dependent task. This field is required when a job consists of more than one task.
    #[serde(rename = "depends_on", skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<crate::models::TaskDependenciesInner>>,
    /// If existing_cluster_id, the ID of an existing cluster that is used for all runs of this task. When running tasks on an existing cluster, you may need to manually restart the cluster if it stops responding. We suggest running jobs on new clusters for greater reliability.
    #[serde(rename = "existing_cluster_id", skip_serializing_if = "Option::is_none")]
    pub existing_cluster_id: Option<String>,
    #[serde(rename = "new_cluster", skip_serializing_if = "Option::is_none")]
    pub new_cluster: Option<Box<crate::models::NewTaskCluster>>,
    /// If job_cluster_key, this task is executed reusing the cluster specified in `job.settings.job_clusters`.
    #[serde(rename = "job_cluster_key", skip_serializing_if = "Option::is_none")]
    pub job_cluster_key: Option<String>,
    #[serde(rename = "notebook_task", skip_serializing_if = "Option::is_none")]
    pub notebook_task: Option<Box<crate::models::NotebookTask>>,
    #[serde(rename = "spark_jar_task", skip_serializing_if = "Option::is_none")]
    pub spark_jar_task: Option<Box<crate::models::SparkJarTask>>,
    #[serde(rename = "spark_python_task", skip_serializing_if = "Option::is_none")]
    pub spark_python_task: Option<Box<crate::models::SparkPythonTask>>,
    #[serde(rename = "spark_submit_task", skip_serializing_if = "Option::is_none")]
    pub spark_submit_task: Option<Box<crate::models::TaskSparkSubmitTask>>,
    #[serde(rename = "pipeline_task", skip_serializing_if = "Option::is_none")]
    pub pipeline_task: Option<Box<crate::models::PipelineTask>>,
    #[serde(rename = "python_wheel_task", skip_serializing_if = "Option::is_none")]
    pub python_wheel_task: Option<crate::models::PythonWheelTask>,
    #[serde(rename = "sql_task", skip_serializing_if = "Option::is_none")]
    pub sql_task: Option<Box<crate::models::SqlTask>>,
    #[serde(rename = "dbt_task", skip_serializing_if = "Option::is_none")]
    pub dbt_task: Option<Box<crate::models::DbtTask>>,
    /// An optional list of libraries to be installed on the cluster that executes the task. The default value is an empty list.
    #[serde(rename = "libraries", skip_serializing_if = "Option::is_none")]
    pub libraries: Option<Vec<crate::models::Library>>,
    #[serde(rename = "email_notifications", skip_serializing_if = "Option::is_none")]
    pub email_notifications: Option<Box<crate::models::JobEmailNotifications>>,
    /// An optional timeout applied to each run of this job task. The default behavior is to have no timeout.
    #[serde(rename = "timeout_seconds", skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    /// An optional maximum number of times to retry an unsuccessful run. A run is considered to be unsuccessful if it completes with the `FAILED` result_state or `INTERNAL_ERROR` `life_cycle_state`. The value -1 means to retry indefinitely and the value 0 means to never retry. The default behavior is to never retry.
    #[serde(rename = "max_retries", skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    /// An optional minimal interval in milliseconds between the start of the failed run and the subsequent retry run. The default behavior is that unsuccessful runs are immediately retried.
    #[serde(rename = "min_retry_interval_millis", skip_serializing_if = "Option::is_none")]
    pub min_retry_interval_millis: Option<i32>,
    /// An optional policy to specify whether to retry a task when it times out. The default behavior is to not retry on timeout.
    #[serde(rename = "retry_on_timeout", skip_serializing_if = "Option::is_none")]
    pub retry_on_timeout: Option<bool>,
}

impl JobTaskSettings {
    pub fn new(task_key: String) -> JobTaskSettings {
        JobTaskSettings {
            task_key,
            description: None,
            depends_on: None,
            existing_cluster_id: None,
            new_cluster: None,
            job_cluster_key: None,
            notebook_task: None,
            spark_jar_task: None,
            spark_python_task: None,
            spark_submit_task: None,
            pipeline_task: None,
            python_wheel_task: None,
            sql_task: None,
            dbt_task: None,
            libraries: None,
            email_notifications: None,
            timeout_seconds: None,
            max_retries: None,
            min_retry_interval_millis: None,
            retry_on_timeout: None,
        }
    }
}


