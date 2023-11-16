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

/// JobsJobSource : The source of the job specification in the remote repository when the job is source controlled.



#[derive(JsonSchema, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobsJobSource {
    /// This describes an enum
    #[serde(rename = "dirty_state", skip_serializing_if = "Option::is_none")]
    pub dirty_state: Option<DirtyState>,
    /// Name of the branch which the job is imported from.
    #[serde(rename = "import_from_git_branch")]
    pub import_from_git_branch: String,
    /// Path of the job YAML file that contains the job specification.
    #[serde(rename = "job_config_path")]
    pub job_config_path: String,
}

impl JobsJobSource {
    /// The source of the job specification in the remote repository when the job is source controlled.
    pub fn new(import_from_git_branch: String, job_config_path: String) -> JobsJobSource {
        JobsJobSource {
            dirty_state: None,
            import_from_git_branch,
            job_config_path,
        }
    }
}

/// This describes an enum
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DirtyState {
    #[serde(rename = "NOT_SYNCED")]
    NotSynced,
    #[serde(rename = "DISCONNECTED")]
    Disconnected,
}

impl Default for DirtyState {
    fn default() -> DirtyState {
        Self::NotSynced
    }
}

