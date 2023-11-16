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
pub struct CatalogTagsSubentityAssignment {
    /// Subentity associated with the tagging information
    #[serde(rename = "securable")]
    pub securable: Box<crate::models::CatalogTagSubentity>,
    /// Name of the subentity
    #[serde(rename = "subentity")]
    pub subentity: String,
    #[serde(rename = "tag_key_value_pairs")]
    pub tag_key_value_pairs: Vec<crate::models::CatalogTagKeyValuePair>,
}

impl CatalogTagsSubentityAssignment {
    pub fn new(securable: crate::models::CatalogTagSubentity, subentity: String, tag_key_value_pairs: Vec<crate::models::CatalogTagKeyValuePair>) -> CatalogTagsSubentityAssignment {
        CatalogTagsSubentityAssignment {
            securable: Box::new(securable),
            subentity,
            tag_key_value_pairs,
        }
    }
}


