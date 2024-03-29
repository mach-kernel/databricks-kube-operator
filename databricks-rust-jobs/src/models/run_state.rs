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

/// RunState : The result and lifecycle state of the run.



#[derive(JsonSchema, Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunState {
    #[serde(rename = "life_cycle_state", skip_serializing_if = "Option::is_none")]
    pub life_cycle_state: Option<crate::models::RunLifeCycleState>,
    #[serde(rename = "result_state", skip_serializing_if = "Option::is_none")]
    pub result_state: Option<crate::models::RunResultState>,
    /// Whether a run was canceled manually by a user or by the scheduler because the run timed out.
    #[serde(rename = "user_cancelled_or_timedout", skip_serializing_if = "Option::is_none")]
    pub user_cancelled_or_timedout: Option<bool>,
    /// A descriptive message for the current state. This field is unstructured, and its exact format is subject to change.
    #[serde(rename = "state_message", skip_serializing_if = "Option::is_none")]
    pub state_message: Option<String>,
}

impl RunState {
    /// The result and lifecycle state of the run.
    pub fn new() -> RunState {
        RunState {
            life_cycle_state: None,
            result_state: None,
            user_cancelled_or_timedout: None,
            state_message: None,
        }
    }
}


