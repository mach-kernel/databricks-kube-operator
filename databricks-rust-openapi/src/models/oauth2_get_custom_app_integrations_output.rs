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
pub struct Oauth2GetCustomAppIntegrationsOutput {
    #[serde(rename = "apps", skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<crate::models::Oauth2GetCustomAppIntegrationOutput>>,
}

impl Oauth2GetCustomAppIntegrationsOutput {
    pub fn new() -> Oauth2GetCustomAppIntegrationsOutput {
        Oauth2GetCustomAppIntegrationsOutput {
            apps: None,
        }
    }
}


