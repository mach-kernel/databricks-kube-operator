# JobsBaseRun

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**creator_user_name** | Option<**String**> | The creator user name. This field won’t be included in the response if the user has already been deleted. | [optional]
**execution_duration** | Option<**i64**> | The time in milliseconds it took to execute the commands in the JAR or notebook until they  completed, failed, timed out, were cancelled, or encountered an unexpected error. The duration of a task run is the sum of the `setup_duration`, `execution_duration`, and the  `cleanup_duration`. The `execution_duration` field is set to 0 for multitask job runs. The total  duration of a multitask job run is the value of the `run_duration` field. | [optional]
**run_type** | Option<[**crate::models::JobsRunType**](JobsRunType.md)> |  | [optional]
**original_attempt_run_id** | Option<**i64**> | If this run is a retry of a prior run attempt, this field contains the run_id of the original attempt; otherwise, it is the same as the run_id. | [optional]
**tasks** | Option<[**Vec<crate::models::JobsRunTask>**](JobsRunTask.md)> |  | [optional]
**job_clusters** | Option<[**Vec<crate::models::JobsJobCluster>**](JobsJobCluster.md)> |  | [optional]
**run_duration** | Option<**i32**> | The time in milliseconds it took the job run and all of its repairs to finish. | [optional]
**cluster_spec** | Option<[**crate::models::JobsClusterSpec**](JobsClusterSpec.md)> | A snapshot of the job’s cluster specification when this run was created. | [optional]
**attempt_number** | Option<**i32**> | The sequence number of this run attempt for a triggered job run. The initial attempt of a run has an attempt_number of 0\\. If the initial run attempt fails, and the job has a retry policy (`max_retries` \\> 0), subsequent runs are created with an `original_attempt_run_id` of the original attempt’s ID and an incrementing `attempt_number`. Runs are retried only until they succeed, and the maximum `attempt_number` is the same as the `max_retries` value for the job. | [optional]
**setup_duration** | Option<**i64**> | The time in milliseconds it took to set up the cluster. For runs that run on new clusters this is the cluster creation time, for runs that run on existing clusters this time should be very short. The duration of a task run is the sum of the `setup_duration`, `execution_duration`, and the `cleanup_duration`. The `setup_duration` field is set to 0 for multitask job runs. The total duration of a multitask job run is the value of the `run_duration` field. | [optional]
**schedule** | Option<[**crate::models::JobsCronSchedule**](JobsCronSchedule.md)> | The cron schedule that triggered this run if it was triggered by the periodic scheduler. | [optional]
**job_parameters** | Option<[**Vec<crate::models::JobsJobParameter>**](JobsJobParameter.md)> |  | [optional]
**job_id** | Option<**i64**> | The canonical identifier of the job that contains this run. | [optional]
**cleanup_duration** | Option<**i64**> | The time in milliseconds it took to terminate the cluster and clean up any associated artifacts. The duration of a task run is the sum of the `setup_duration`, `execution_duration`, and the `cleanup_duration`. The `cleanup_duration` field is set to 0 for multitask job runs. The total duration of a multitask job run is the value of the `run_duration` field. | [optional]
**number_in_job** | Option<**i64**> | A unique identifier for this job run. This is set to the same value as `run_id`. | [optional]
**overriding_parameters** | Option<[**crate::models::JobsRunParameters**](JobsRunParameters.md)> | The parameters used for this run. | [optional]
**end_time** | Option<**i64**> | The time at which this run ended in epoch milliseconds (milliseconds since 1/1/1970 UTC). This field is set to 0 if the job is still running. | [optional]
**run_id** | Option<**i64**> | The canonical identifier of the run. This ID is unique across all runs of all jobs. | [optional]
**state** | Option<[**crate::models::JobsRunState**](JobsRunState.md)> |  | [optional]
**run_page_url** | Option<**String**> | The URL to the detail page of the run. | [optional]
**git_source** | Option<[**crate::models::JobsGitSource**](JobsGitSource.md)> | An optional specification for a remote Git repository containing the source code used by tasks. Version-controlled source code is supported by notebook, dbt, Python script, and SQL File tasks.  If `git_source` is set, these tasks retrieve the file from the remote repository by default. However, this behavior can be overridden by setting `source` to `WORKSPACE` on the task.  Note: dbt and SQL File tasks support only version-controlled sources. If dbt or SQL File tasks are used, `git_source` must be defined on the job. | [optional]
**continuous** | Option<[**crate::models::JobsContinuous**](JobsContinuous.md)> | The continuous trigger that triggered this run. | [optional]
**trigger_info** | Option<[**crate::models::JobsTriggerInfo**](JobsTriggerInfo.md)> |  | [optional]
**start_time** | Option<**i64**> | The time at which this run was started in epoch milliseconds (milliseconds since 1/1/1970 UTC). This may not be the time when the job task starts executing, for example, if the job is scheduled to run on a new cluster, this is the time the cluster creation call is issued. | [optional]
**trigger** | Option<[**crate::models::JobsTriggerType**](JobsTriggerType.md)> |  | [optional]
**run_name** | Option<**String**> | An optional name for the run. The maximum length is 4096 bytes in UTF-8 encoding. | [optional][default to Untitled]
**cluster_instance** | Option<[**crate::models::JobsClusterInstance**](JobsClusterInstance.md)> | The cluster used for this run. If the run is specified to use a new cluster, this field is set once the Jobs service has requested a cluster for the run. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


