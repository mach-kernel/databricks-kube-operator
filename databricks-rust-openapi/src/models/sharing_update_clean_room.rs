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
pub struct SharingUpdateCleanRoom {
    #[serde(rename = "catalog_updates", skip_serializing_if = "Option::is_none")]
    pub catalog_updates: Option<Vec<crate::models::SharingCleanRoomCatalogUpdate>>,
    /// User-provided free-form text description.
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Name of the clean room.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Username of current owner of clean room.
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

impl SharingUpdateCleanRoom {
    pub fn new() -> SharingUpdateCleanRoom {
        SharingUpdateCleanRoom {
            catalog_updates: None,
            comment: None,
            name: None,
            owner: None,
        }
    }
}


