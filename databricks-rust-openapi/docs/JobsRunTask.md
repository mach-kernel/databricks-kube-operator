# JobsRunTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**execution_duration** | Option<**i64**> | The time in milliseconds it took to execute the commands in the JAR or notebook until they  completed, failed, timed out, were cancelled, or encountered an unexpected error. The duration of a task run is the sum of the `setup_duration`, `execution_duration`, and the  `cleanup_duration`. The `execution_duration` field is set to 0 for multitask job runs. The total  duration of a multitask job run is the value of the `run_duration` field. | [optional]
**dbt_task** | Option<[**crate::models::JobsDbtTask**](JobsDbtTask.md)> | If dbt_task, indicates that this must execute a dbt task. It requires both Databricks SQL and the ability to use a serverless or a pro SQL warehouse. | [optional]
**condition_task** | Option<[**crate::models::JobsRunConditionTask**](JobsRunConditionTask.md)> | If condition_task, specifies a condition with an outcome that can be used to control the execution of other tasks. Does not require a cluster to execute and does not support retries or notifications. | [optional]
**sql_task** | Option<[**crate::models::JobsSqlTask**](JobsSqlTask.md)> | If sql_task, indicates that this job must execute a SQL. | [optional]
**libraries** | Option<[**Vec<crate::models::ComputeLibrary>**](ComputeLibrary.md)> |  | [optional]
**notebook_task** | Option<[**crate::models::JobsNotebookTask**](JobsNotebookTask.md)> | If notebook_task, indicates that this job must run a notebook. This field may not be specified in conjunction with spark_jar_task. | [optional]
**python_wheel_task** | Option<[**crate::models::JobsPythonWheelTask**](JobsPythonWheelTask.md)> | If python_wheel_task, indicates that this job must execute a PythonWheel. | [optional]
**attempt_number** | Option<**i32**> | The sequence number of this run attempt for a triggered job run. The initial attempt of a run has an attempt_number of 0\\. If the initial run attempt fails, and the job has a retry policy (`max_retries` \\> 0), subsequent runs are created with an `original_attempt_run_id` of the original attempt’s ID and an incrementing `attempt_number`. Runs are retried only until they succeed, and the maximum `attempt_number` is the same as the `max_retries` value for the job. | [optional]
**spark_python_task** | Option<[**crate::models::JobsSparkPythonTask**](JobsSparkPythonTask.md)> | If spark_python_task, indicates that this job must run a Python file. | [optional]
**resolved_values** | Option<[**crate::models::JobsResolvedValues**](JobsResolvedValues.md)> | Parameter values including resolved references | [optional]
**setup_duration** | Option<**i64**> | The time in milliseconds it took to set up the cluster. For runs that run on new clusters this is the cluster creation time, for runs that run on existing clusters this time should be very short. The duration of a task run is the sum of the `setup_duration`, `execution_duration`, and the `cleanup_duration`. The `setup_duration` field is set to 0 for multitask job runs. The total duration of a multitask job run is the value of the `run_duration` field. | [optional]
**spark_jar_task** | Option<[**crate::models::JobsSparkJarTask**](JobsSparkJarTask.md)> | If spark_jar_task, indicates that this job must run a JAR. | [optional]
**task_key** | Option<**String**> | A unique name for the task. This field is used to refer to this task from other tasks. This field is required and must be unique within its parent job. On Update or Reset, this field is used to reference the tasks to be updated or reset. | [optional]
**cleanup_duration** | Option<**i64**> | The time in milliseconds it took to terminate the cluster and clean up any associated artifacts. The duration of a task run is the sum of the `setup_duration`, `execution_duration`, and the `cleanup_duration`. The `cleanup_duration` field is set to 0 for multitask job runs. The total duration of a multitask job run is the value of the `run_duration` field. | [optional]
**depends_on** | Option<[**Vec<crate::models::JobsTaskDependency>**](JobsTaskDependency.md)> |  | [optional]
**run_if** | Option<[**crate::models::JobsRunIf**](JobsRunIf.md)> | An optional value indicating the condition that determines whether the task should be run once its dependencies have been completed. When omitted, defaults to `ALL_SUCCESS`. See :method:jobs/create for a list of possible values. | [optional]
**end_time** | Option<**i64**> | The time at which this run ended in epoch milliseconds (milliseconds since 1/1/1970 UTC). This field is set to 0 if the job is still running. | [optional]
**run_id** | Option<**i64**> | The ID of the task run. | [optional]
**state** | Option<[**crate::models::JobsRunState**](JobsRunState.md)> |  | [optional]
**pipeline_task** | Option<[**crate::models::JobsPipelineTask**](JobsPipelineTask.md)> | If pipeline_task, indicates that this job must execute a Pipeline. | [optional]
**git_source** | Option<[**crate::models::JobsGitSource**](JobsGitSource.md)> | An optional specification for a remote Git repository containing the source code used by tasks. Version-controlled source code is supported by notebook, dbt, Python script, and SQL File tasks.  If `git_source` is set, these tasks retrieve the file from the remote repository by default. However, this behavior can be overridden by setting `source` to `WORKSPACE` on the task.  Note: dbt and SQL File tasks support only version-controlled sources. If dbt or SQL File tasks are used, `git_source` must be defined on the job. | [optional]
**run_job_task** | Option<[**crate::models::JobsRunJobTask**](JobsRunJobTask.md)> | If run_job_task, indicates that this task must execute another job. | [optional]
**new_cluster** | Option<[**crate::models::ComputeClusterSpec**](ComputeClusterSpec.md)> | If new_cluster, a description of a new cluster that is created only for this task. | [optional]
**start_time** | Option<**i64**> | The time at which this run was started in epoch milliseconds (milliseconds since 1/1/1970 UTC). This may not be the time when the job task starts executing, for example, if the job is scheduled to run on a new cluster, this is the time the cluster creation call is issued. | [optional]
**description** | Option<**String**> | An optional description for this task. | [optional]
**spark_submit_task** | Option<[**crate::models::JobsSparkSubmitTask**](JobsSparkSubmitTask.md)> | If spark_submit_task, indicates that this task must be launched by the spark submit script. This task can run only on new clusters | [optional]
**cluster_instance** | Option<[**crate::models::JobsClusterInstance**](JobsClusterInstance.md)> | The cluster used for this run. If the run is specified to use a new cluster, this field is set once the Jobs service has requested a cluster for the run. | [optional]
**existing_cluster_id** | Option<**String**> | If existing_cluster_id, the ID of an existing cluster that is used for all runs of this job. When running jobs on an existing cluster, you may need to manually restart the cluster if it stops responding. We suggest running jobs on new clusters for greater reliability. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


