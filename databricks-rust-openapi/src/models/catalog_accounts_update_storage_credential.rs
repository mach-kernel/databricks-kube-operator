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
pub struct CatalogAccountsUpdateStorageCredential {
    #[serde(rename = "credential_info", skip_serializing_if = "Option::is_none")]
    pub credential_info: Option<Box<crate::models::CatalogUpdateStorageCredential>>,
}

impl CatalogAccountsUpdateStorageCredential {
    pub fn new() -> CatalogAccountsUpdateStorageCredential {
        CatalogAccountsUpdateStorageCredential {
            credential_info: None,
        }
    }
}


