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

/// CanManageRun : Permission to run and/or manage runs for the job.

/// Permission to run and/or manage runs for the job.
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CanManageRun {
    #[serde(rename = "CAN_MANAGE_RUN")]
    CanManageRun,

}

impl ToString for CanManageRun {
    fn to_string(&self) -> String {
        match self {
            Self::CanManageRun => String::from("CAN_MANAGE_RUN"),
        }
    }
}

impl Default for CanManageRun {
    fn default() -> CanManageRun {
        Self::CanManageRun
    }
}




