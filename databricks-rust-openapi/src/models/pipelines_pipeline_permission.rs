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
pub struct PipelinesPipelinePermission {
    /// 
    #[serde(rename = "inherited", skip_serializing_if = "Option::is_none")]
    pub inherited: Option<bool>,
    #[serde(rename = "inherited_from_object", skip_serializing_if = "Option::is_none")]
    pub inherited_from_object: Option<Vec<String>>,
    #[serde(rename = "permission_level", skip_serializing_if = "Option::is_none")]
    pub permission_level: Option<crate::models::PipelinesPipelinePermissionLevel>,
}

impl PipelinesPipelinePermission {
    pub fn new() -> PipelinesPipelinePermission {
        PipelinesPipelinePermission {
            inherited: None,
            inherited_from_object: None,
            permission_level: None,
        }
    }
}


