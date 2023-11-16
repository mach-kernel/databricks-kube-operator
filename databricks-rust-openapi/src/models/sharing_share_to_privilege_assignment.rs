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
pub struct SharingShareToPrivilegeAssignment {
    #[serde(rename = "privilege_assignments", skip_serializing_if = "Option::is_none")]
    pub privilege_assignments: Option<Vec<crate::models::SharingPrivilegeAssignment>>,
    /// The share name.
    #[serde(rename = "share_name", skip_serializing_if = "Option::is_none")]
    pub share_name: Option<String>,
}

impl SharingShareToPrivilegeAssignment {
    pub fn new() -> SharingShareToPrivilegeAssignment {
        SharingShareToPrivilegeAssignment {
            privilege_assignments: None,
            share_name: None,
        }
    }
}


