# Run

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**job_id** | Option<**i64**> | The canonical identifier of the job that contains this run. | [optional]
**run_id** | Option<**i64**> | The canonical identifier of the run. This ID is unique across all runs of all jobs. | [optional]
**number_in_job** | Option<**i64**> | A unique identifier for this job run. This is set to the same value as `run_id`. | [optional]
**creator_user_name** | Option<**String**> | The creator user name. This field won’t be included in the response if the user has already been deleted. | [optional]
**original_attempt_run_id** | Option<**i64**> | If this run is a retry of a prior run attempt, this field contains the run_id of the original attempt; otherwise, it is the same as the run_id. | [optional]
**state** | Option<[**crate::models::RunState**](RunState.md)> |  | [optional]
**schedule** | Option<[**crate::models::CronSchedule**](CronSchedule.md)> |  | [optional]
**continuous** | Option<[**crate::models::Continuous**](Continuous.md)> |  | [optional]
**tasks** | Option<[**Vec<crate::models::RunTask>**](RunTask.md)> | The list of tasks performed by the run. Each task has its own `run_id` which you can use to call `JobsGetOutput` to retrieve the run resutls. | [optional]
**job_clusters** | Option<[**Vec<crate::models::JobCluster>**](JobCluster.md)> | A list of job cluster specifications that can be shared and reused by tasks of this job. Libraries cannot be declared in a shared job cluster. You must declare dependent libraries in task settings. | [optional]
**cluster_spec** | Option<[**crate::models::ClusterSpec**](ClusterSpec.md)> |  | [optional]
**cluster_instance** | Option<[**crate::models::ClusterInstance**](ClusterInstance.md)> |  | [optional]
**git_source** | Option<[**crate::models::GitSource**](GitSource.md)> |  | [optional]
**overriding_parameters** | Option<[**crate::models::RunParameters**](RunParameters.md)> |  | [optional]
**start_time** | Option<**i64**> | The time at which this run was started in epoch milliseconds (milliseconds since 1/1/1970 UTC). This may not be the time when the job task starts executing, for example, if the job is scheduled to run on a new cluster, this is the time the cluster creation call is issued. | [optional]
**setup_duration** | Option<**i64**> | The time in milliseconds it took to set up the cluster. For runs that run on new clusters this is the cluster creation time, for runs that run on existing clusters this time should be very short. The duration of a task run is the sum of the `setup_duration`, `execution_duration`, and the `cleanup_duration`. The `setup_duration` field is set to 0 for multitask job runs. The total duration of a multitask job run is the value of the `run_duration` field. | [optional]
**execution_duration** | Option<**i64**> | The time in milliseconds it took to execute the commands in the JAR or notebook until they  completed, failed, timed out, were cancelled, or encountered an unexpected error. The duration of a task run is the sum of the `setup_duration`, `execution_duration`, and the  `cleanup_duration`. The `execution_duration` field is set to 0 for multitask job runs. The total  duration of a multitask job run is the value of the `run_duration` field. | [optional]
**cleanup_duration** | Option<**i64**> | The time in milliseconds it took to terminate the cluster and clean up any associated artifacts. The duration of a task run is the sum of the `setup_duration`, `execution_duration`, and the `cleanup_duration`. The `cleanup_duration` field is set to 0 for multitask job runs. The total duration of a multitask job run is the value of the `run_duration` field. | [optional]
**end_time** | Option<**i64**> | The time at which this run ended in epoch milliseconds (milliseconds since 1/1/1970 UTC). This field is set to 0 if the job is still running. | [optional]
**run_duration** | Option<**i64**> | The time in milliseconds it took the job run and all of its repairs to finish. This field is only set for multitask job runs and not task runs. The duration of a task run is the sum of the `setup_duration`, `execution_duration`, and the `cleanup_duration`. | [optional]
**trigger** | Option<[**crate::models::TriggerType**](TriggerType.md)> |  | [optional]
**run_name** | Option<**String**> | An optional name for the run. The maximum allowed length is 4096 bytes in UTF-8 encoding. | [optional][default to Untitled]
**run_page_url** | Option<**String**> | The URL to the detail page of the run. | [optional]
**run_type** | Option<[**crate::models::RunType**](RunType.md)> |  | [optional]
**attempt_number** | Option<**i32**> | The sequence number of this run attempt for a triggered job run. The initial attempt of a run has an attempt_number of 0\\. If the initial run attempt fails, and the job has a retry policy (`max_retries` \\> 0), subsequent runs are created with an `original_attempt_run_id` of the original attempt’s ID and an incrementing `attempt_number`. Runs are retried only until they succeed, and the maximum `attempt_number` is the same as the `max_retries` value for the job. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


