# JobsRunsGetOutput200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**notebook_output** | Option<[**crate::models::NotebookOutput**](NotebookOutput.md)> |  | [optional]
**sql_output** | Option<[**crate::models::SqlOutput**](SqlOutput.md)> |  | [optional]
**dbt_output** | Option<[**crate::models::DbtOutput**](DbtOutput.md)> |  | [optional]
**logs** | Option<**String**> | The output from tasks that write to standard streams (stdout/stderr) such as [SparkJarTask](https://docs.databricks.com/dev-tools/api/latest/jobs.html#/components/schemas/SparkJarTask), [SparkPythonTask](https://docs.databricks.com/dev-tools/api/latest/jobs.html#/components/schemas/SparkPythonTask, [PythonWheelTask](https://docs.databricks.com/dev-tools/api/latest/jobs.html#/components/schemas/PythonWheelTask. It's not supported for the [NotebookTask](https://docs.databricks.com/dev-tools/api/latest/jobs.html#/components/schemas/NotebookTask, [PipelineTask](https://docs.databricks.com/dev-tools/api/latest/jobs.html#/components/schemas/PipelineTask, or [SparkSubmitTask](https://docs.databricks.com/dev-tools/api/latest/jobs.html#/components/schemas/SparkSubmitTask. Databricks restricts this API to return the last 5 MB of these logs. | [optional]
**logs_truncated** | Option<**bool**> | Whether the logs are truncated. | [optional]
**error** | Option<**String**> | An error message indicating why a task failed or why output is not available. The message is unstructured, and its exact format is subject to change. | [optional]
**error_trace** | Option<**String**> | If there was an error executing the run, this field contains any available stack traces. | [optional]
**metadata** | Option<[**crate::models::Run**](Run.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


