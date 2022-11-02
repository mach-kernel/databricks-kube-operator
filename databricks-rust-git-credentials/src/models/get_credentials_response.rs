use schemars::JsonSchema;
/*
 * Git Credentials API
 *
 * The Git credentials API allows users to manage their [Git credentials](https://docs.databricks.com/repos.html#configure-your-git-integration-with-databricks) to use [Databricks Repos](https://docs.databricks.com/repos.html).
 *
 * The version of the OpenAPI document: 2.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(JsonSchema, Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCredentialsResponse {
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Vec<crate::models::GetCredentialResponse>>,
}

impl GetCredentialsResponse {
    pub fn new() -> GetCredentialsResponse {
        GetCredentialsResponse { credentials: None }
    }
}
