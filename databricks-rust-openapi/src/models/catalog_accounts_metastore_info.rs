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
pub struct CatalogAccountsMetastoreInfo {
    #[serde(rename = "metastore_info", skip_serializing_if = "Option::is_none")]
    pub metastore_info: Option<Box<crate::models::CatalogMetastoreInfo>>,
}

impl CatalogAccountsMetastoreInfo {
    pub fn new() -> CatalogAccountsMetastoreInfo {
        CatalogAccountsMetastoreInfo {
            metastore_info: None,
        }
    }
}


