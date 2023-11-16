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
pub struct ComputeLogAnalyticsInfo {
    /// <needs content added>
    #[serde(rename = "log_analytics_primary_key", skip_serializing_if = "Option::is_none")]
    pub log_analytics_primary_key: Option<String>,
    /// <needs content added>
    #[serde(rename = "log_analytics_workspace_id", skip_serializing_if = "Option::is_none")]
    pub log_analytics_workspace_id: Option<String>,
}

impl ComputeLogAnalyticsInfo {
    pub fn new() -> ComputeLogAnalyticsInfo {
        ComputeLogAnalyticsInfo {
            log_analytics_primary_key: None,
            log_analytics_workspace_id: None,
        }
    }
}


