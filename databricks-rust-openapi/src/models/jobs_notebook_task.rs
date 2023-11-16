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
pub struct JobsNotebookTask {
    /// Base parameters to be used for each run of this job. If the run is initiated by a call to :method:jobs/runNow with parameters specified, the two parameters maps are merged. If the same key is specified in `base_parameters` and in `run-now`, the value from `run-now` is used.  Use [Task parameter variables](https://Docsdatabricks.com/jobs.html#parameter-variables) to set parameters containing information about job runs.  If the notebook takes a parameter that is not specified in the job’s `base_parameters` or the `run-now` override parameters, the default value from the notebook is used.  Retrieve these parameters in a notebook using [dbutils.widgets.get](https://Docsdatabricks.com/dev-tools/databricks-utils.html#dbutils-widgets). 
    #[serde(rename = "base_parameters", default, skip_serializing_if = "Option::is_none")]
    pub base_parameters: Option<::std::collections::HashMap<String, String>>,
    /// The path of the notebook to be run in the Databricks workspace or remote repository. For notebooks stored in the Databricks workspace, the path must be absolute and begin with a slash. For notebooks stored in a remote repository, the path must be relative. This field is required. 
    #[serde(rename = "notebook_path")]
    pub notebook_path: String,
    /// Optional location type of the notebook. When set to `WORKSPACE`, the notebook will be retrieved from the local <Databricks> workspace. When set to `GIT`, the notebook will be retrieved from a Git repository defined in `git_source`. If the value is empty, the task will use `GIT` if `git_source` is defined and `WORKSPACE` otherwise.  * `WORKSPACE`: Notebook is located in <Databricks> workspace. * `GIT`: Notebook is located in cloud Git provider. 
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::models::JobsSource>,
}

impl JobsNotebookTask {
    pub fn new(notebook_path: String) -> JobsNotebookTask {
        JobsNotebookTask {
            base_parameters: None,
            notebook_path,
            source: None,
        }
    }
}


