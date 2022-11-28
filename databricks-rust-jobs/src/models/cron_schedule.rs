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
pub struct CronSchedule {
    /// A Cron expression using Quartz syntax that describes the schedule for a job. See [Cron Trigger](http://www.quartz-scheduler.org/documentation/quartz-2.3.0/tutorials/crontrigger.html) for details. This field is required.
    #[serde(rename = "quartz_cron_expression")]
    pub quartz_cron_expression: String,
    /// A Java timezone ID. The schedule for a job is resolved with respect to this timezone. See [Java TimeZone](https://docs.oracle.com/javase/7/docs/api/java/util/TimeZone.html) for details. This field is required.
    #[serde(rename = "timezone_id")]
    pub timezone_id: String,
    /// Indicate whether this schedule is paused or not.
    #[serde(rename = "pause_status", skip_serializing_if = "Option::is_none")]
    pub pause_status: Option<PauseStatus>,
}

impl CronSchedule {
    pub fn new(quartz_cron_expression: String, timezone_id: String) -> CronSchedule {
        CronSchedule {
            quartz_cron_expression,
            timezone_id,
            pause_status: None,
        }
    }
}

/// Indicate whether this schedule is paused or not.
#[derive(
    JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum PauseStatus {
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "UNPAUSED")]
    Unpaused,
}

impl Default for PauseStatus {
    fn default() -> PauseStatus {
        Self::Paused
    }
}