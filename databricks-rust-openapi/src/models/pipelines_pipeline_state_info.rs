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
pub struct PipelinesPipelineStateInfo {
    /// The unique identifier of the cluster running the pipeline.
    #[serde(rename = "cluster_id", skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// The username of the pipeline creator.
    #[serde(rename = "creator_user_name", skip_serializing_if = "Option::is_none")]
    pub creator_user_name: Option<String>,
    #[serde(rename = "latest_updates", skip_serializing_if = "Option::is_none")]
    pub latest_updates: Option<Vec<crate::models::PipelinesUpdateStateInfo>>,
    /// The user-friendly name of the pipeline.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The unique identifier of the pipeline.
    #[serde(rename = "pipeline_id", skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,
    /// The username that the pipeline runs as. This is a read only value derived from the pipeline owner.
    #[serde(rename = "run_as_user_name", skip_serializing_if = "Option::is_none")]
    pub run_as_user_name: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::PipelinesPipelineState>,
}

impl PipelinesPipelineStateInfo {
    pub fn new() -> PipelinesPipelineStateInfo {
        PipelinesPipelineStateInfo {
            cluster_id: None,
            creator_user_name: None,
            latest_updates: None,
            name: None,
            pipeline_id: None,
            run_as_user_name: None,
            state: None,
        }
    }
}


