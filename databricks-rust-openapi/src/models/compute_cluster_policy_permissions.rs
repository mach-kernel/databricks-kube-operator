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
pub struct ComputeClusterPolicyPermissions {
    #[serde(rename = "access_control_list", skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<Vec<crate::models::ComputeClusterPolicyAccessControlResponse>>,
    /// 
    #[serde(rename = "object_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    /// 
    #[serde(rename = "object_type", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
}

impl ComputeClusterPolicyPermissions {
    pub fn new() -> ComputeClusterPolicyPermissions {
        ComputeClusterPolicyPermissions {
            access_control_list: None,
            object_id: None,
            object_type: None,
        }
    }
}


