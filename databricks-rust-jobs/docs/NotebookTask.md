# NotebookTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**notebook_path** | **String** | The path of the notebook to be run in the Databricks workspace or remote repository. For notebooks stored in the Databricks workspace, the path must be absolute and begin with a slash. For notebooks stored in a remote repository, the path must be relative. This field is required. | 
**source** | Option<**String**> | Optional location type of the notebook. When set to `WORKSPACE`, the notebook will be retrieved from the local Databricks workspace. When set to `GIT`, the notebook will be retrieved from a Git repository defined in `git_source`. If the value is empty, the task will use `GIT` if `git_source` is defined and `WORKSPACE` otherwise. | [optional]
**base_parameters** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Base parameters to be used for each run of this job. If the run is initiated by a call to [`run-now`](https://docs.databricks.com/dev-tools/api/latest/jobs.html#operation/JobsRunNow) with parameters specified, the two parameters maps are merged. If the same key is specified in `base_parameters` and in `run-now`, the value from `run-now` is used.  Use [Task parameter variables](https://docs.databricks.com/jobs.html#parameter-variables) to set parameters containing information about job runs.  If the notebook takes a parameter that is not specified in the job’s `base_parameters` or the `run-now` override parameters, the default value from the notebook is used.  Retrieve these parameters in a notebook using [dbutils.widgets.get](https://docs.databricks.com/dev-tools/databricks-utils.html#dbutils-widgets). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


