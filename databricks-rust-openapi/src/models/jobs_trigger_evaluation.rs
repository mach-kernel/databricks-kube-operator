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
pub struct JobsTriggerEvaluation {
    /// Human-readable description of the the trigger evaluation result. Explains why the trigger evaluation triggered or did not trigger a run, or failed.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the run that was triggered by the trigger evaluation. Only returned if a run was triggered.
    #[serde(rename = "run_id", skip_serializing_if = "Option::is_none")]
    pub run_id: Option<i64>,
    /// Timestamp at which the trigger was evaluated.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

impl JobsTriggerEvaluation {
    pub fn new() -> JobsTriggerEvaluation {
        JobsTriggerEvaluation {
            description: None,
            run_id: None,
            timestamp: None,
        }
    }
}


