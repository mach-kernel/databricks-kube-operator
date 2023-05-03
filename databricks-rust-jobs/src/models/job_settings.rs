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
pub struct JobSettings {
    /// An optional name for the job.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A map of tags associated with the job. These are forwarded to the cluster as cluster tags for jobs clusters, and are subject to the same limitations as cluster tags. A maximum of 25 tags can be added to the job.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    /// A list of task specifications to be executed by this job.
    #[serde(rename = "tasks", skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<crate::models::JobTaskSettings>>,
    /// A list of job cluster specifications that can be shared and reused by tasks of this job. Libraries cannot be declared in a shared job cluster. You must declare dependent libraries in task settings.
    #[serde(rename = "job_clusters", skip_serializing_if = "Option::is_none")]
    pub job_clusters: Option<Vec<crate::models::JobCluster>>,
    #[serde(rename = "email_notifications", skip_serializing_if = "Option::is_none")]
    pub email_notifications: Option<Box<crate::models::JobEmailNotifications>>,
    #[serde(rename = "webhook_notifications", skip_serializing_if = "Option::is_none")]
    pub webhook_notifications: Option<Box<crate::models::WebhookNotifications>>,
    /// An optional timeout applied to each run of this job. The default behavior is to have no timeout.
    #[serde(rename = "timeout_seconds", skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Box<crate::models::CronSchedule>>,
    #[serde(rename = "continuous", skip_serializing_if = "Option::is_none")]
    pub continuous: Option<Box<crate::models::Continuous>>,
    /// An optional maximum allowed number of concurrent runs of the job.  Set this value if you want to be able to execute multiple runs of the same job concurrently. This is useful for example if you trigger your job on a frequent schedule and want to allow consecutive runs to overlap with each other, or if you want to trigger multiple runs which differ by their input parameters.  This setting affects only new runs. For example, suppose the job’s concurrency is 4 and there are 4 concurrent active runs. Then setting the concurrency to 3 won’t kill any of the active runs. However, from then on, new runs are skipped unless there are fewer than 3 active runs.  This value cannot exceed 1000\\. Setting this value to 0 causes all new runs to be skipped. The default behavior is to allow only 1 concurrent run.
    #[serde(rename = "max_concurrent_runs", skip_serializing_if = "Option::is_none")]
    pub max_concurrent_runs: Option<i32>,
    #[serde(rename = "git_source", skip_serializing_if = "Option::is_none")]
    pub git_source: Option<Box<crate::models::GitSource>>,
    /// Used to tell what is the format of the job. This field is ignored in Create/Update/Reset calls. When using the Jobs API 2.1 this value is always set to `\"MULTI_TASK\"`.
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<Format>,
}

impl JobSettings {
    pub fn new() -> JobSettings {
        JobSettings {
            name: None,
            tags: None,
            tasks: None,
            job_clusters: None,
            email_notifications: None,
            webhook_notifications: None,
            timeout_seconds: None,
            schedule: None,
            continuous: None,
            max_concurrent_runs: None,
            git_source: None,
            format: None,
        }
    }
}

/// Used to tell what is the format of the job. This field is ignored in Create/Update/Reset calls. When using the Jobs API 2.1 this value is always set to `\"MULTI_TASK\"`.
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "SINGLE_TASK")]
    SingleTask,
    #[serde(rename = "MULTI_TASK")]
    MultiTask,
}

impl Default for Format {
    fn default() -> Format {
        Self::SingleTask
    }
}

