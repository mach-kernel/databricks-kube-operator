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
pub struct SharingCleanRoomTableInfo {
    /// Name of parent catalog.
    #[serde(rename = "catalog_name", skip_serializing_if = "Option::is_none")]
    pub catalog_name: Option<String>,
    #[serde(rename = "columns", skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<crate::models::SharingColumnInfo>>,
    /// Full name of table, in form of __catalog_name__.__schema_name__.__table_name__
    #[serde(rename = "full_name", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// Name of table, relative to parent schema.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Name of parent schema relative to its parent catalog.
    #[serde(rename = "schema_name", skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
}

impl SharingCleanRoomTableInfo {
    pub fn new() -> SharingCleanRoomTableInfo {
        SharingCleanRoomTableInfo {
            catalog_name: None,
            columns: None,
            full_name: None,
            name: None,
            schema_name: None,
        }
    }
}


