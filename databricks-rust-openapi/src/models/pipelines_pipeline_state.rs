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

/// PipelinesPipelineState : The pipeline state.

/// The pipeline state.
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PipelinesPipelineState {
    #[serde(rename = "DEPLOYING")]
    Deploying,
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "STOPPING")]
    Stopping,
    #[serde(rename = "DELETED")]
    Deleted,
    #[serde(rename = "RECOVERING")]
    Recovering,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "RESETTING")]
    Resetting,
    #[serde(rename = "IDLE")]
    Idle,

}

impl ToString for PipelinesPipelineState {
    fn to_string(&self) -> String {
        match self {
            Self::Deploying => String::from("DEPLOYING"),
            Self::Starting => String::from("STARTING"),
            Self::Running => String::from("RUNNING"),
            Self::Stopping => String::from("STOPPING"),
            Self::Deleted => String::from("DELETED"),
            Self::Recovering => String::from("RECOVERING"),
            Self::Failed => String::from("FAILED"),
            Self::Resetting => String::from("RESETTING"),
            Self::Idle => String::from("IDLE"),
        }
    }
}

impl Default for PipelinesPipelineState {
    fn default() -> PipelinesPipelineState {
        Self::Deploying
    }
}




