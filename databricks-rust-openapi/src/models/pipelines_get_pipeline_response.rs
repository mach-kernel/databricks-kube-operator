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
pub struct PipelinesGetPipelineResponse {
    /// The username of the pipeline creator.
    #[serde(rename = "creator_user_name", skip_serializing_if = "Option::is_none")]
    pub creator_user_name: Option<String>,
    /// The ID of the pipeline.
    #[serde(rename = "pipeline_id", skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,
    /// Username of the user that the pipeline will run on behalf of.
    #[serde(rename = "run_as_user_name", skip_serializing_if = "Option::is_none")]
    pub run_as_user_name: Option<String>,
    /// The pipeline specification. This field is not returned when called by `ListPipelines`.
    #[serde(rename = "spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<crate::models::PipelinesPipelineSpec>>,
    /// The health of a pipeline.
    #[serde(rename = "health", skip_serializing_if = "Option::is_none")]
    pub health: Option<Health>,
    /// The last time the pipeline settings were modified or created.
    #[serde(rename = "last_modified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<i64>,
    /// A human friendly identifier for the pipeline, taken from the `spec`.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// An optional message detailing the cause of the pipeline state.
    #[serde(rename = "cause", skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    /// The ID of the cluster that the pipeline is running on.
    #[serde(rename = "cluster_id", skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::PipelinesPipelineState>,
    #[serde(rename = "latest_updates", skip_serializing_if = "Option::is_none")]
    pub latest_updates: Option<Vec<crate::models::PipelinesUpdateStateInfo>>,
}

impl PipelinesGetPipelineResponse {
    pub fn new() -> PipelinesGetPipelineResponse {
        PipelinesGetPipelineResponse {
            creator_user_name: None,
            pipeline_id: None,
            run_as_user_name: None,
            spec: None,
            health: None,
            last_modified: None,
            name: None,
            cause: None,
            cluster_id: None,
            state: None,
            latest_updates: None,
        }
    }
}

/// The health of a pipeline.
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Health {
    #[serde(rename = "HEALTHY")]
    Healthy,
    #[serde(rename = "UNHEALTHY")]
    Unhealthy,
}

impl Default for Health {
    fn default() -> Health {
        Self::Healthy
    }
}

