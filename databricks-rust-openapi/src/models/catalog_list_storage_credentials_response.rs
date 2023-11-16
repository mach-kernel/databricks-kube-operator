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
pub struct CatalogListStorageCredentialsResponse {
    #[serde(rename = "storage_credentials", skip_serializing_if = "Option::is_none")]
    pub storage_credentials: Option<Vec<crate::models::CatalogStorageCredentialInfo>>,
}

impl CatalogListStorageCredentialsResponse {
    pub fn new() -> CatalogListStorageCredentialsResponse {
        CatalogListStorageCredentialsResponse {
            storage_credentials: None,
        }
    }
}


