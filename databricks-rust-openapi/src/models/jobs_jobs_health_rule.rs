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
pub struct JobsJobsHealthRule {
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: Option<crate::models::JobsJobsHealthMetric>,
    #[serde(rename = "op", skip_serializing_if = "Option::is_none")]
    pub op: Option<crate::models::JobsJobsHealthOperator>,
    /// Specifies the threshold value that the health metric should obey to satisfy the health rule.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

impl JobsJobsHealthRule {
    pub fn new() -> JobsJobsHealthRule {
        JobsJobsHealthRule {
            metric: None,
            op: None,
            value: None,
        }
    }
}


