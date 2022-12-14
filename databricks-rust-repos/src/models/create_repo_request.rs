use schemars::JsonSchema;
/*
 * Repos API
 *
 * The repos API allows users to manage their [repos](https://docs.databricks.com/repos.html). Users can use the API to access all repos that they have manage permissions on.
 *
 * The version of the OpenAPI document: 2.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(JsonSchema, Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateRepoRequest {
    /// URL of the Git repository to be linked.
    #[serde(rename = "url")]
    pub url: String,
    /// Git provider. This field is case-insensitive. The available Git providers are gitHub, bitbucketCloud, gitLab, azureDevOpsServices, gitHubEnterprise, bitbucketServer, gitLabEnterpriseEdition and awsCodeCommit.
    #[serde(rename = "provider")]
    pub provider: String,
    /// Desired path for the repo in the workspace. Must be in the format /Repos/{folder}/{repo-name}.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

impl CreateRepoRequest {
    pub fn new(url: String, provider: String) -> CreateRepoRequest {
        CreateRepoRequest {
            url,
            provider,
            path: None,
        }
    }
}
