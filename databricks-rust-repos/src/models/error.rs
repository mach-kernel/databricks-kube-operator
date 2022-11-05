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
pub struct Error {
    /// Error code
    #[serde(rename = "error_code", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// Human-readable error message describing the cause of the error.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl Error {
    pub fn new() -> Error {
        Error {
            error_code: None,
            message: None,
        }
    }
}
