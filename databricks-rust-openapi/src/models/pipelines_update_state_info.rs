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
pub struct PipelinesUpdateStateInfo {
    /// 
    #[serde(rename = "creation_time", skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// 
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// 
    #[serde(rename = "update_id", skip_serializing_if = "Option::is_none")]
    pub update_id: Option<String>,
}

impl PipelinesUpdateStateInfo {
    pub fn new() -> PipelinesUpdateStateInfo {
        PipelinesUpdateStateInfo {
            creation_time: None,
            state: None,
            update_id: None,
        }
    }
}

/// 
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "QUEUED")]
    Queued,
    #[serde(rename = "CREATED")]
    Created,
    #[serde(rename = "WAITING_FOR_RESOURCES")]
    WaitingForResources,
    #[serde(rename = "INITIALIZING")]
    Initializing,
    #[serde(rename = "RESETTING")]
    Resetting,
    #[serde(rename = "SETTING_UP_TABLES")]
    SettingUpTables,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "STOPPING")]
    Stopping,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "CANCELED")]
    Canceled,
}

impl Default for State {
    fn default() -> State {
        Self::Queued
    }
}

