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




#[derive(JsonSchema, Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceRepoInfo {
    /// Branch that the local version of the repo is checked out to.
    #[serde(rename = "branch", skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// SHA-1 hash representing the commit ID of the current HEAD of the repo.
    #[serde(rename = "head_commit_id", skip_serializing_if = "Option::is_none")]
    pub head_commit_id: Option<String>,
    /// ID of the repo object in the workspace.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Desired path for the repo in the workspace. Must be in the format /Repos/{folder}/{repo-name}.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Git provider. This field is case-insensitive. The available Git providers are gitHub, bitbucketCloud, gitLab, azureDevOpsServices, gitHubEnterprise, bitbucketServer, gitLabEnterpriseEdition and awsCodeCommit.
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "sparse_checkout", skip_serializing_if = "Option::is_none")]
    pub sparse_checkout: Option<Box<crate::models::WorkspaceSparseCheckout>>,
    /// URL of the Git repository to be linked.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl WorkspaceRepoInfo {
    pub fn new() -> WorkspaceRepoInfo {
        WorkspaceRepoInfo {
            branch: None,
            head_commit_id: None,
            id: None,
            path: None,
            provider: None,
            sparse_checkout: None,
            url: None,
        }
    }
}


