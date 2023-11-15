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
pub struct UpdateRepoRequest {
    /// Branch that the local version of the repo is checked out to.
    #[serde(rename = "branch")]
    pub branch: String,
    /// Tag that the local version of the repo is checked out to. Updating the repo to a tag puts the repo in a detached HEAD state. Before committing new changes, you must update the repo to a branch instead of the detached HEAD.
    #[serde(rename = "tag")]
    pub tag: String,
}

impl UpdateRepoRequest {
    pub fn new(branch: String, tag: String) -> UpdateRepoRequest {
        UpdateRepoRequest { branch, tag }
    }
}