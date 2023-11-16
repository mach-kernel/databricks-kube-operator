# JobsRepairRun

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rerun_all_failed_tasks** | Option<**bool**> | If true, repair all failed tasks. Only one of `rerun_tasks` or `rerun_all_failed_tasks` can be used. | [optional][default to false]
**latest_repair_id** | Option<**i64**> | The ID of the latest repair. This parameter is not required when repairing a run for the first time, but must be provided on subsequent requests to repair the same run. | [optional]
**notebook_params** | Option<**::std::collections::HashMap<String, String>**> | A map from keys to values for jobs with notebook task, for example `\\\"notebook_params\\\": {\\\"name\\\": \\\"john doe\\\", \\\"age\\\": \\\"35\\\"}`. The map is passed to the notebook and is accessible through the [Dbutilswidgets.get](https://docs.databricks.com/dev-tools/databricks-utils.html) function.  If not specified upon `run-now`, the triggered run uses the job’s base parameters.  notebook_params cannot be specified in conjunction with jar_params.  Use [Task parameter variables](https://docs.databricks.com/jobs.html#parameter-variables) to set parameters containing information about job runs.  The JSON representation of this field (for example `{\\\"notebook_params\\\":{\\\"name\\\":\\\"john doe\\\",\\\"age\\\":\\\"35\\\"}}`) cannot exceed 10,000 bytes.  | [optional]
**dbt_commands** | Option<**Vec<String>**> |  | [optional]
**python_named_params** | Option<**::std::collections::HashMap<String, String>**> | A map from keys to values for jobs with Python wheel task, for example `\"python_named_params\": {\"name\": \"task\", \"data\": \"dbfs:/path/to/Datajson\"}`. | [optional]
**sql_params** | Option<**::std::collections::HashMap<String, String>**> | A map from keys to values for jobs with SQL task, for example `\"sql_params\": {\"name\": \"john doe\", \"age\": \"35\"}`. The SQL alert task does not support custom parameters. | [optional]
**rerun_dependent_tasks** | Option<**bool**> | If true, repair all tasks that depend on the tasks in `rerun_tasks`, even if they were previously successful. Can be also used in combination with `rerun_all_failed_tasks`. | [optional][default to false]
**pipeline_params** | Option<[**crate::models::JobsPipelineParams**](JobsPipelineParams.md)> |  | [optional]
**spark_submit_params** | Option<**Vec<String>**> |  | [optional]
**rerun_tasks** | Option<**Vec<String>**> |  | [optional]
**run_id** | **i64** | The job run ID of the run to repair. The run must not be in progress. | 
**jar_params** | Option<**Vec<String>**> |  | [optional]
**python_params** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


