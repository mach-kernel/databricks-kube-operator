# JobsTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dbt_task** | Option<[**crate::models::JobsDbtTask**](JobsDbtTask.md)> | If dbt_task, indicates that this must execute a dbt task. It requires both Databricks SQL and the ability to use a serverless or a pro SQL warehouse. | [optional]
**condition_task** | Option<[**crate::models::JobsConditionTask**](JobsConditionTask.md)> | If condition_task, specifies a condition with an outcome that can be used to control the execution of other tasks. Does not require a cluster to execute and does not support retries or notifications. | [optional]
**sql_task** | Option<[**crate::models::JobsSqlTask**](JobsSqlTask.md)> | If sql_task, indicates that this job must execute a SQL task. | [optional]
**libraries** | Option<[**Vec<crate::models::ComputeLibrary>**](ComputeLibrary.md)> |  | [optional]
**timeout_seconds** | Option<**i32**> | An optional timeout applied to each run of this job task. The default behavior is to have no timeout. | [optional]
**retry_on_timeout** | Option<**bool**> | An optional policy to specify whether to retry a task when it times out. The default behavior is to not retry on timeout. | [optional]
**notebook_task** | Option<[**crate::models::JobsNotebookTask**](JobsNotebookTask.md)> | If notebook_task, indicates that this task must run a notebook. This field may not be specified in conjunction with spark_jar_task. | [optional]
**health** | Option<[**crate::models::JobsJobsHealthRules**](JobsJobsHealthRules.md)> |  | [optional]
**max_retries** | Option<**i32**> | An optional maximum number of times to retry an unsuccessful run. A run is considered to be unsuccessful if it completes with the `FAILED` result_state or `INTERNAL_ERROR` `life_cycle_state`. The value -1 means to retry indefinitely and the value 0 means to never retry. The default behavior is to never retry. | [optional]
**python_wheel_task** | Option<[**crate::models::JobsPythonWheelTask**](JobsPythonWheelTask.md)> | If python_wheel_task, indicates that this job must execute a PythonWheel. | [optional]
**spark_python_task** | Option<[**crate::models::JobsSparkPythonTask**](JobsSparkPythonTask.md)> | If spark_python_task, indicates that this task must run a Python file. | [optional]
**email_notifications** | Option<[**crate::models::JobsTaskEmailNotifications**](JobsTaskEmailNotifications.md)> | An optional set of email addresses that is notified when runs of this task begin or complete as well as when this task is deleted. The default behavior is to not send any emails. | [optional]
**spark_jar_task** | Option<[**crate::models::JobsSparkJarTask**](JobsSparkJarTask.md)> | If spark_jar_task, indicates that this task must run a JAR. | [optional]
**task_key** | **String** | A unique name for the task. This field is used to refer to this task from other tasks. This field is required and must be unique within its parent job. On Update or Reset, this field is used to reference the tasks to be updated or reset. | 
**job_cluster_key** | Option<**String**> | If job_cluster_key, this task is executed reusing the cluster specified in `Jobsettings.job_clusters`. | [optional]
**depends_on** | Option<[**Vec<crate::models::JobsTaskDependency>**](JobsTaskDependency.md)> |  | [optional]
**run_if** | Option<[**crate::models::JobsRunIf**](JobsRunIf.md)> | An optional value specifying the condition determining whether the task is run once its dependencies have been completed. When omitted, defaults to `ALL_SUCCESS`.  * `ALL_SUCCESS`: All dependencies have executed and succeeded * `AT_LEAST_ONE_SUCCESS`: At least one dependency has succeeded * `NONE_FAILED`: None of the dependencies have failed and at least one was executed * `ALL_DONE`: All dependencies have been completed * `AT_LEAST_ONE_FAILED`: At least one dependency failed * `ALL_FAILED`: ALl dependencies have failed  | [optional]
**compute_key** | Option<**String**> | The key of the compute requirement, specified in `Jobsettings.compute`, to use for execution of this task. | [optional]
**pipeline_task** | Option<[**crate::models::JobsPipelineTask**](JobsPipelineTask.md)> | If pipeline_task, indicates that this task must execute a Pipeline. | [optional]
**notification_settings** | Option<[**crate::models::JobsTaskNotificationSettings**](JobsTaskNotificationSettings.md)> | Optional notification settings that are used when sending notifications to each of the `email_notifications` for this task. | [optional]
**run_job_task** | Option<[**crate::models::JobsRunJobTask**](JobsRunJobTask.md)> | If run_job_task, indicates that this task must execute another job. | [optional]
**min_retry_interval_millis** | Option<**i32**> | An optional minimal interval in milliseconds between the start of the failed run and the subsequent retry run. The default behavior is that unsuccessful runs are immediately retried. | [optional]
**new_cluster** | Option<[**crate::models::ComputeClusterSpec**](ComputeClusterSpec.md)> | If new_cluster, a description of a cluster that is created for only for this task. | [optional]
**description** | Option<**String**> | An optional description for this task. | [optional]
**spark_submit_task** | Option<[**crate::models::JobsSparkSubmitTask**](JobsSparkSubmitTask.md)> | If spark_submit_task, indicates that this task must be launched by the spark submit script. This task can run only on new clusters. | [optional]
**existing_cluster_id** | Option<**String**> | If existing_cluster_id, the ID of an existing cluster that is used for all runs of this task. When running tasks on an existing cluster, you may need to manually restart the cluster if it stops responding. We suggest running jobs on new clusters for greater reliability. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


