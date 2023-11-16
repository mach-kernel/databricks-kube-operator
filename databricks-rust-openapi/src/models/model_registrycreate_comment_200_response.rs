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
pub struct ModelRegistrycreateComment200Response {
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<Box<crate::models::MlCommentObject>>,
}

impl ModelRegistrycreateComment200Response {
    pub fn new() -> ModelRegistrycreateComment200Response {
        ModelRegistrycreateComment200Response {
            comment: None,
        }
    }
}


