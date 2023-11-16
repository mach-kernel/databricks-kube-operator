# JobsJobCluster

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**job_cluster_key** | **String** | A unique name for the job cluster. This field is required and must be unique within the job. `JobTaskSettings` may refer to this field to determine which cluster to launch for the task execution. | 
**new_cluster** | Option<[**crate::models::ComputeClusterSpec**](ComputeClusterSpec.md)> | If new_cluster, a description of a cluster that is created for each task. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


