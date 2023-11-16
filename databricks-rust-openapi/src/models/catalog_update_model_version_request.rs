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
pub struct CatalogUpdateModelVersionRequest {
    /// The comment attached to the model version
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl CatalogUpdateModelVersionRequest {
    pub fn new() -> CatalogUpdateModelVersionRequest {
        CatalogUpdateModelVersionRequest {
            comment: None,
        }
    }
}


