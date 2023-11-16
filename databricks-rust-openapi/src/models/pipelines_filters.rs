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
pub struct PipelinesFilters {
    #[serde(rename = "exclude", skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    #[serde(rename = "include", skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}

impl PipelinesFilters {
    pub fn new() -> PipelinesFilters {
        PipelinesFilters {
            exclude: None,
            include: None,
        }
    }
}


