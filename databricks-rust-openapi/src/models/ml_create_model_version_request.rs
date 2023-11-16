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
pub struct MlCreateModelVersionRequest {
    /// Optional description for model version.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Register model under this name
    #[serde(rename = "name")]
    pub name: String,
    /// MLflow run ID for correlation, if `source` was generated by an experiment run in MLflow tracking server
    #[serde(rename = "run_id", skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// MLflow run link - this is the exact link of the run that generated this model version, potentially hosted at another instance of MLflow.
    #[serde(rename = "run_link", skip_serializing_if = "Option::is_none")]
    pub run_link: Option<String>,
    /// URI indicating the location of the model artifacts.
    #[serde(rename = "source")]
    pub source: String,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::MlModelVersionTag>>,
}

impl MlCreateModelVersionRequest {
    pub fn new(name: String, source: String) -> MlCreateModelVersionRequest {
        MlCreateModelVersionRequest {
            description: None,
            name,
            run_id: None,
            run_link: None,
            source,
            tags: None,
        }
    }
}


