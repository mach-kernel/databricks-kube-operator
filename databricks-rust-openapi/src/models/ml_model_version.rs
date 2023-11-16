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
pub struct MlModelVersion {
    /// User that created this `model_version`.
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::MlModelVersionTag>>,
    /// Unique name of the model
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Current status of `model_version`
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Run Link: Direct link to the run that generated this version
    #[serde(rename = "run_link", skip_serializing_if = "Option::is_none")]
    pub run_link: Option<String>,
    /// URI indicating the location of the source model artifacts, used when creating `model_version`
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// MLflow run ID used when creating `model_version`, if `source` was generated by an experiment run stored in MLflow tracking server.
    #[serde(rename = "run_id", skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    /// Details on current `status`, if it is pending or failed.
    #[serde(rename = "status_message", skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// Model's version number.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Timestamp recorded when metadata for this `model_version` was last updated.
    #[serde(rename = "last_updated_timestamp", skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<i64>,
    /// Timestamp recorded when this `model_version` was created.
    #[serde(rename = "creation_timestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<i64>,
    /// Description of this `model_version`.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Current stage for this `model_version`.
    #[serde(rename = "current_stage", skip_serializing_if = "Option::is_none")]
    pub current_stage: Option<String>,
}

impl MlModelVersion {
    pub fn new() -> MlModelVersion {
        MlModelVersion {
            user_id: None,
            tags: None,
            name: None,
            status: None,
            run_link: None,
            source: None,
            run_id: None,
            status_message: None,
            version: None,
            last_updated_timestamp: None,
            creation_timestamp: None,
            description: None,
            current_stage: None,
        }
    }
}

/// Current status of `model_version`
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "PENDING_REGISTRATION")]
    PendingRegistration,
    #[serde(rename = "FAILED_REGISTRATION")]
    FailedRegistration,
    #[serde(rename = "READY")]
    Ready,
}

impl Default for Status {
    fn default() -> Status {
        Self::PendingRegistration
    }
}

