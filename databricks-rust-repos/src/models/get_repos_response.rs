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
pub struct GetReposResponse {
    #[serde(rename = "repos", skip_serializing_if = "Option::is_none")]
    pub repos: Option<Vec<crate::models::GetRepoResponse>>,
    /// Token that can be specified as a query parameter to the GET /repos endpoint to retrieve the next page of results.
    #[serde(rename = "next_page_token", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

impl GetReposResponse {
    pub fn new() -> GetReposResponse {
        GetReposResponse {
            repos: None,
            next_page_token: None,
        }
    }
}
