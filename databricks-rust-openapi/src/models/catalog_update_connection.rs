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
pub struct CatalogUpdateConnection {
    /// Name of the connection.
    #[serde(rename = "name")]
    pub name: String,
    /// A map of key-value properties attached to the securable.
    #[serde(rename = "options", deserialize_with = "Option::deserialize")]
    pub options: Option<::std::collections::HashMap<String, String>>,
}

impl CatalogUpdateConnection {
    pub fn new(name: String, options: Option<::std::collections::HashMap<String, String>>) -> CatalogUpdateConnection {
        CatalogUpdateConnection {
            name,
            options,
        }
    }
}


