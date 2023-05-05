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

#[derive(JsonSchema, Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NotebookTask {
    /// The path of the notebook to be run in the Databricks workspace or remote repository. For notebooks stored in the Databricks workspace, the path must be absolute and begin with a slash. For notebooks stored in a remote repository, the path must be relative. This field is required.
    #[serde(rename = "notebook_path")]
    pub notebook_path: String,
    /// Optional location type of the notebook. When set to `WORKSPACE`, the notebook will be retrieved from the local Databricks workspace. When set to `GIT`, the notebook will be retrieved from a Git repository defined in `git_source`. If the value is empty, the task will use `GIT` if `git_source` is defined and `WORKSPACE` otherwise.
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// Base parameters to be used for each run of this job. If the run is initiated by a call to [`run-now`](https://docs.databricks.com/dev-tools/api/latest/jobs.html#operation/JobsRunNow) with parameters specified, the two parameters maps are merged. If the same key is specified in `base_parameters` and in `run-now`, the value from `run-now` is used.  Use [Task parameter variables](https://docs.databricks.com/jobs.html#parameter-variables) to set parameters containing information about job runs.  If the notebook takes a parameter that is not specified in the job’s `base_parameters` or the `run-now` override parameters, the default value from the notebook is used.  Retrieve these parameters in a notebook using [dbutils.widgets.get](https://docs.databricks.com/dev-tools/databricks-utils.html#dbutils-widgets).
    #[serde(rename = "base_parameters", skip_serializing_if = "Option::is_none")]
    pub base_parameters: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl NotebookTask {
    pub fn new(notebook_path: String) -> NotebookTask {
        NotebookTask {
            notebook_path,
            source: None,
            base_parameters: None,
        }
    }
}

/// Optional location type of the notebook. When set to `WORKSPACE`, the notebook will be retrieved from the local Databricks workspace. When set to `GIT`, the notebook will be retrieved from a Git repository defined in `git_source`. If the value is empty, the task will use `GIT` if `git_source` is defined and `WORKSPACE` otherwise.
#[derive(
    JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum Source {
    #[serde(rename = "WORKSPACE")]
    Workspace,
    #[serde(rename = "GIT")]
    Git,
}

impl Default for Source {
    fn default() -> Source {
        Self::Workspace
    }
}
