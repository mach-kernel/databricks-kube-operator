# RunSubmitTaskSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**task_key** | **String** | A unique name for the task. This field is used to refer to this task from other tasks. This field is required and must be unique within its parent job. On Update or Reset, this field is used to reference the tasks to be updated or reset. The maximum length is 100 characters. | 
**depends_on** | Option<[**Vec<crate::models::TaskDependenciesInner>**](TaskDependencies_inner.md)> | An optional array of objects specifying the dependency graph of the task. All tasks specified in this field must complete successfully before executing this task. The key is `task_key`, and the value is the name assigned to the dependent task. This field is required when a job consists of more than one task. | [optional]
**existing_cluster_id** | Option<**String**> | If existing_cluster_id, the ID of an existing cluster that is used for all runs of this task. When running tasks on an existing cluster, you may need to manually restart the cluster if it stops responding. We suggest running jobs on new clusters for greater reliability. | [optional]
**new_cluster** | Option<[**crate::models::NewCluster**](NewCluster.md)> |  | [optional]
**notebook_task** | Option<[**crate::models::NotebookTask**](NotebookTask.md)> |  | [optional]
**spark_jar_task** | Option<[**crate::models::SparkJarTask**](SparkJarTask.md)> |  | [optional]
**spark_python_task** | Option<[**crate::models::SparkPythonTask**](SparkPythonTask.md)> |  | [optional]
**spark_submit_task** | Option<[**crate::models::SparkSubmitTask**](SparkSubmitTask.md)> |  | [optional]
**pipeline_task** | Option<[**crate::models::PipelineTask**](PipelineTask.md)> |  | [optional]
**python_wheel_task** | Option<[**crate::models::PythonWheelTask**](PythonWheelTask.md)> |  | [optional]
**sql_task** | Option<[**crate::models::SqlTask**](SqlTask.md)> |  | [optional]
**libraries** | Option<[**Vec<crate::models::Library>**](Library.md)> | An optional list of libraries to be installed on the cluster that executes the task. The default value is an empty list. | [optional]
**timeout_seconds** | Option<**i32**> | An optional timeout applied to each run of this job task. The default behavior is to have no timeout. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


