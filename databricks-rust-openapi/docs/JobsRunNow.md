# JobsRunNow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**notebook_params** | Option<**::std::collections::HashMap<String, String>**> | A map from keys to values for jobs with notebook task, for example `\\\"notebook_params\\\": {\\\"name\\\": \\\"john doe\\\", \\\"age\\\": \\\"35\\\"}`. The map is passed to the notebook and is accessible through the [Dbutilswidgets.get](https://docs.databricks.com/dev-tools/databricks-utils.html) function.  If not specified upon `run-now`, the triggered run uses the job’s base parameters.  notebook_params cannot be specified in conjunction with jar_params.  Use [Task parameter variables](https://docs.databricks.com/jobs.html#parameter-variables) to set parameters containing information about job runs.  The JSON representation of this field (for example `{\\\"notebook_params\\\":{\\\"name\\\":\\\"john doe\\\",\\\"age\\\":\\\"35\\\"}}`) cannot exceed 10,000 bytes.  | [optional]
**dbt_commands** | Option<**Vec<String>**> |  | [optional]
**python_named_params** | Option<**::std::collections::HashMap<String, String>**> | A map from keys to values for jobs with Python wheel task, for example `\"python_named_params\": {\"name\": \"task\", \"data\": \"dbfs:/path/to/Datajson\"}`. | [optional]
**idempotency_token** | Option<**String**> | An optional token to guarantee the idempotency of job run requests. If a run with the provided token already exists, the request does not create a new run but returns the ID of the existing run instead. If a run with the provided token is deleted, an error is returned.  If you specify the idempotency token, upon failure you can retry until the request succeeds. Databricks guarantees that exactly one run is launched with that idempotency token.  This token must have at most 64 characters.  For more information, see [How to ensure idempotency for jobs]( https://Kbdatabricks.com/jobs/jobs-idempotency.html). | [optional]
**sql_params** | Option<**::std::collections::HashMap<String, String>**> | A map from keys to values for jobs with SQL task, for example `\"sql_params\": {\"name\": \"john doe\", \"age\": \"35\"}`. The SQL alert task does not support custom parameters. | [optional]
**pipeline_params** | Option<[**crate::models::JobsPipelineParams**](JobsPipelineParams.md)> |  | [optional]
**spark_submit_params** | Option<**Vec<String>**> |  | [optional]
**job_parameters** | Option<[**Vec<::std::collections::HashMap<String, String>>**](map.md)> |  | [optional]
**job_id** | **i64** | The ID of the job to be executed | 
**jar_params** | Option<**Vec<String>**> |  | [optional]
**python_params** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


