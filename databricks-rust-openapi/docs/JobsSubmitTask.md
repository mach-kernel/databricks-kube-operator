# JobsSubmitTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**condition_task** | Option<[**crate::models::JobsConditionTask**](JobsConditionTask.md)> | If condition_task, specifies a condition with an outcome that can be used to control the execution of other tasks. Does not require a cluster to execute and does not support retries or notifications. | [optional]
**sql_task** | Option<[**crate::models::JobsSqlTask**](JobsSqlTask.md)> | If sql_task, indicates that this job must execute a SQL. | [optional]
**libraries** | Option<[**Vec<crate::models::ComputeLibrary>**](ComputeLibrary.md)> |  | [optional]
**timeout_seconds** | Option<**i32**> | An optional timeout applied to each run of this job task. The default behavior is to have no timeout. | [optional]
**notebook_task** | Option<[**crate::models::JobsNotebookTask**](JobsNotebookTask.md)> | If notebook_task, indicates that this task must run a notebook. This field may not be specified in conjunction with spark_jar_task. | [optional]
**health** | Option<[**crate::models::JobsJobsHealthRules**](JobsJobsHealthRules.md)> |  | [optional]
**python_wheel_task** | Option<[**crate::models::JobsPythonWheelTask**](JobsPythonWheelTask.md)> | If python_wheel_task, indicates that this job must execute a PythonWheel. | [optional]
**spark_python_task** | Option<[**crate::models::JobsSparkPythonTask**](JobsSparkPythonTask.md)> | If spark_python_task, indicates that this task must run a Python file. | [optional]
**email_notifications** | Option<[**crate::models::JobsJobEmailNotifications**](JobsJobEmailNotifications.md)> | An optional set of email addresses notified when the task run begins or completes. The default behavior is to not send any emails. | [optional]
**spark_jar_task** | Option<[**crate::models::JobsSparkJarTask**](JobsSparkJarTask.md)> | If spark_jar_task, indicates that this task must run a JAR. | [optional]
**task_key** | **String** | A unique name for the task. This field is used to refer to this task from other tasks. This field is required and must be unique within its parent job. On Update or Reset, this field is used to reference the tasks to be updated or reset. | 
**depends_on** | Option<[**Vec<crate::models::JobsTaskDependency>**](JobsTaskDependency.md)> |  | [optional]
**pipeline_task** | Option<[**crate::models::JobsPipelineTask**](JobsPipelineTask.md)> | If pipeline_task, indicates that this task must execute a Pipeline. | [optional]
**notification_settings** | Option<[**crate::models::JobsTaskNotificationSettings**](JobsTaskNotificationSettings.md)> | Optional notification settings that are used when sending email notifications for this task run. | [optional]
**new_cluster** | Option<[**crate::models::ComputeClusterSpec**](ComputeClusterSpec.md)> | If new_cluster, a description of a cluster that is created for each run. | [optional]
**spark_submit_task** | Option<[**crate::models::JobsSparkSubmitTask**](JobsSparkSubmitTask.md)> | If spark_submit_task, indicates that this task must be launched by the spark submit script. This task can run only on new clusters. | [optional]
**existing_cluster_id** | Option<**String**> | If existing_cluster_id, the ID of an existing cluster that is used for all runs of this task. When running tasks on an existing cluster, you may need to manually restart the cluster if it stops responding. We suggest running jobs on new clusters for greater reliability. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


