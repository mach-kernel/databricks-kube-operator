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
pub struct SharingUpdateShare {
    /// User-provided free-form text description.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Name of the share.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Username of current owner of share.
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "updates", skip_serializing_if = "Option::is_none")]
    pub updates: Option<Vec<crate::models::SharingSharedDataObjectUpdate>>,
}

impl SharingUpdateShare {
    pub fn new() -> SharingUpdateShare {
        SharingUpdateShare {
            comment: None,
            name: None,
            owner: None,
            updates: None,
        }
    }
}


