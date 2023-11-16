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

/// JobsJobsHealthMetric : Specifies the health metric that is being evaluated for a particular health rule.

/// Specifies the health metric that is being evaluated for a particular health rule.
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JobsJobsHealthMetric {
    #[serde(rename = "RUN_DURATION_SECONDS")]
    RunDurationSeconds,

}

impl ToString for JobsJobsHealthMetric {
    fn to_string(&self) -> String {
        match self {
            Self::RunDurationSeconds => String::from("RUN_DURATION_SECONDS"),
        }
    }
}

impl Default for JobsJobsHealthMetric {
    fn default() -> JobsJobsHealthMetric {
        Self::RunDurationSeconds
    }
}




