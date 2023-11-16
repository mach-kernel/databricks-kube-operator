# JobsRunOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**condition_task** | Option<[**serde_json::Value**](.md)> |  | [optional]
**error** | Option<**String**> | An error message indicating why a task failed or why output is not available. The message is unstructured, and its exact format is subject to change. | [optional]
**notebook_output** | Option<[**crate::models::JobsNotebookOutput**](JobsNotebookOutput.md)> | The output of a notebook task, if available. A notebook task that terminates (either successfully or with a failure) without calling `Dbutilsnotebook.exit()` is considered to have an empty output. This field is set but its result value is empty. <Databricks> restricts this API to return the first 5 MB of the output. To return a larger result, use the [ClusterLogConf](/dev-tools/api/latest/clusters.html#clusterlogconf) field to configure log storage for the job cluster.  | [optional]
**dbt_output** | Option<[**crate::models::JobsDbtOutput**](JobsDbtOutput.md)> | The output of a dbt task, if available. | [optional]
**sql_output** | Option<[**crate::models::JobsSqlOutput**](JobsSqlOutput.md)> | The output of a SQL task, if available. | [optional]
**run_job_output** | Option<[**crate::models::JobsRunJobOutput**](JobsRunJobOutput.md)> | The output of a run job task, if available | [optional]
**logs** | Option<**String**> | The output from tasks that write to standard streams (stdout/stderr) such as spark_jar_task, spark_python_task, python_wheel_task.  It's not supported for the notebook_task, pipeline_task or spark_submit_task.  Databricks restricts this API to return the last 5 MB of these logs.  | [optional]
**metadata** | Option<[**crate::models::JobsRun**](JobsRun.md)> | All details of the run except for its output. | [optional]
**error_trace** | Option<**String**> | If there was an error executing the run, this field contains any available stack traces. | [optional]
**logs_truncated** | Option<**bool**> | Whether the logs are truncated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


