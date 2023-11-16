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
pub struct WorkspaceCreateCredentialsResponse {
    /// ID of the credential object in the workspace.
    #[serde(rename = "credential_id", skip_serializing_if = "Option::is_none")]
    pub credential_id: Option<i64>,
    /// Git provider. This field is case-insensitive. The available Git providers are gitHub, bitbucketCloud, gitLab, azureDevOpsServices, gitHubEnterprise, bitbucketServer, gitLabEnterpriseEdition and awsCodeCommit.
    #[serde(rename = "git_provider", skip_serializing_if = "Option::is_none")]
    pub git_provider: Option<String>,
    /// Git username.
    #[serde(rename = "git_username", skip_serializing_if = "Option::is_none")]
    pub git_username: Option<String>,
}

impl WorkspaceCreateCredentialsResponse {
    pub fn new() -> WorkspaceCreateCredentialsResponse {
        WorkspaceCreateCredentialsResponse {
            credential_id: None,
            git_provider: None,
            git_username: None,
        }
    }
}


