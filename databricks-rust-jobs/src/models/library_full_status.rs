use schemars::JsonSchema;
/*
 * Jobs API 2.1
 *
 * The Jobs API allows you to create, edit, and delete jobs. You should never hard code secrets or store them in plain text. Use the [Secrets API](https://docs.databricks.com/dev-tools/api/latest/secrets.html) to manage secrets in the [Databricks CLI](https://docs.databricks.com/dev-tools/cli/index.html). Use the [Secrets utility](https://docs.databricks.com/dev-tools/databricks-utils.html#dbutils-secrets) to reference secrets in notebooks and jobs.
 *
 * The version of the OpenAPI document: 2.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(JsonSchema, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LibraryFullStatus {
    #[serde(rename = "library", skip_serializing_if = "Option::is_none")]
    pub library: Option<Box<crate::models::Library>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::LibraryInstallStatus>,
    /// All the info and warning messages that have occurred so far for this library.
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<String>>,
    /// Whether the library was set to be installed on all clusters via the libraries UI.
    #[serde(rename = "is_library_for_all_clusters", skip_serializing_if = "Option::is_none")]
    pub is_library_for_all_clusters: Option<bool>,
}

impl LibraryFullStatus {
    pub fn new() -> LibraryFullStatus {
        LibraryFullStatus {
            library: None,
            status: None,
            messages: None,
            is_library_for_all_clusters: None,
        }
    }
}


