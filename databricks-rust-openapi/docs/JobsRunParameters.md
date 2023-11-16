# JobsRunParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dbt_commands** | Option<**Vec<String>**> |  | [optional]
**jar_params** | Option<**Vec<String>**> |  | [optional]
**notebook_params** | Option<**::std::collections::HashMap<String, String>**> | A map from keys to values for jobs with notebook task, for example `\\\"notebook_params\\\": {\\\"name\\\": \\\"john doe\\\", \\\"age\\\": \\\"35\\\"}`. The map is passed to the notebook and is accessible through the [Dbutilswidgets.get](https://docs.databricks.com/dev-tools/databricks-utils.html) function.  If not specified upon `run-now`, the triggered run uses the job’s base parameters.  notebook_params cannot be specified in conjunction with jar_params.  Use [Task parameter variables](https://docs.databricks.com/jobs.html#parameter-variables) to set parameters containing information about job runs.  The JSON representation of this field (for example `{\\\"notebook_params\\\":{\\\"name\\\":\\\"john doe\\\",\\\"age\\\":\\\"35\\\"}}`) cannot exceed 10,000 bytes.  | [optional]
**pipeline_params** | Option<[**crate::models::JobsPipelineParams**](JobsPipelineParams.md)> |  | [optional]
**python_named_params** | Option<**::std::collections::HashMap<String, String>**> | A map from keys to values for jobs with Python wheel task, for example `\"python_named_params\": {\"name\": \"task\", \"data\": \"dbfs:/path/to/Datajson\"}`. | [optional]
**python_params** | Option<**Vec<String>**> |  | [optional]
**spark_submit_params** | Option<**Vec<String>**> |  | [optional]
**sql_params** | Option<**::std::collections::HashMap<String, String>**> | A map from keys to values for jobs with SQL task, for example `\"sql_params\": {\"name\": \"john doe\", \"age\": \"35\"}`. The SQL alert task does not support custom parameters. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


