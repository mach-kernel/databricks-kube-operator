# RunTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**run_id** | Option<**i64**> | The ID of the task run. | [optional]
**task_key** | Option<**String**> | A unique name for the task. This field is used to refer to this task from other tasks. This field is required and must be unique within its parent job. On Update or Reset, this field is used to reference the tasks to be updated or reset. The maximum length is 100 characters. | [optional]
**description** | Option<**String**> | An optional description for this task. The maximum length is 4096 bytes. | [optional]
**state** | Option<[**crate::models::RunState**](RunState.md)> |  | [optional]
**depends_on** | Option<[**Vec<crate::models::TaskDependenciesInner>**](TaskDependencies_inner.md)> | An optional array of objects specifying the dependency graph of the task. All tasks specified in this field must complete successfully before executing this task. The key is `task_key`, and the value is the name assigned to the dependent task. This field is required when a job consists of more than one task. | [optional]
**existing_cluster_id** | Option<**String**> | If existing_cluster_id, the ID of an existing cluster that is used for all runs of this job. When running jobs on an existing cluster, you may need to manually restart the cluster if it stops responding. We suggest running jobs on new clusters for greater reliability. | [optional]
**new_cluster** | Option<[**crate::models::NewTaskCluster**](NewTaskCluster.md)> |  | [optional]
**libraries** | Option<[**Vec<crate::models::Library>**](Library.md)> | An optional list of libraries to be installed on the cluster that executes the job. The default value is an empty list. | [optional]
**notebook_task** | Option<[**crate::models::NotebookTask**](NotebookTask.md)> |  | [optional]
**spark_jar_task** | Option<[**crate::models::SparkJarTask**](SparkJarTask.md)> |  | [optional]
**spark_python_task** | Option<[**crate::models::SparkPythonTask**](SparkPythonTask.md)> |  | [optional]
**spark_submit_task** | Option<[**crate::models::TaskSparkSubmitTask**](TaskSparkSubmitTask.md)> |  | [optional]
**pipeline_task** | Option<[**crate::models::PipelineTask**](PipelineTask.md)> |  | [optional]
**python_wheel_task** | Option<[**crate::models::PythonWheelTask**](PythonWheelTask.md)> |  | [optional]
**sql_task** | Option<[**crate::models::SqlTask**](SqlTask.md)> |  | [optional]
**dbt_task** | Option<[**crate::models::DbtTask**](DbtTask.md)> |  | [optional]
**start_time** | Option<**i64**> | The time at which this run was started in epoch milliseconds (milliseconds since 1/1/1970 UTC). This may not be the time when the job task starts executing, for example, if the job is scheduled to run on a new cluster, this is the time the cluster creation call is issued. | [optional]
**setup_duration** | Option<**i64**> | The time in milliseconds it took to set up the cluster. For runs that run on new clusters this is the cluster creation time, for runs that run on existing clusters this time should be very short. The duration of a task run is the sum of the `setup_duration`, `execution_duration`, and the `cleanup_duration`. The `setup_duration` field is set to 0 for multitask job runs. The total duration of a multitask job run is the value of the `run_duration` field. | [optional]
**execution_duration** | Option<**i64**> | The time in milliseconds it took to execute the commands in the JAR or notebook until they completed, failed, timed out, were cancelled, or encountered an unexpected error. | [optional]
**cleanup_duration** | Option<**i64**> | The time in milliseconds it took to terminate the cluster and clean up any associated artifacts. The total duration of the run is the sum of the setup_duration, the execution_duration, and the cleanup_duration. | [optional]
**end_time** | Option<**i64**> | The time at which this run ended in epoch milliseconds (milliseconds since 1/1/1970 UTC). This field is set to 0 if the job is still running. | [optional]
**attempt_number** | Option<**i32**> | The sequence number of this run attempt for a triggered job run. The initial attempt of a run has an attempt_number of 0\\. If the initial run attempt fails, and the job has a retry policy (`max_retries` \\> 0), subsequent runs are created with an `original_attempt_run_id` of the original attempt’s ID and an incrementing `attempt_number`. Runs are retried only until they succeed, and the maximum `attempt_number` is the same as the `max_retries` value for the job. | [optional]
**cluster_instance** | Option<[**crate::models::ClusterInstance**](ClusterInstance.md)> |  | [optional]
**git_source** | Option<[**crate::models::GitSource**](GitSource.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


