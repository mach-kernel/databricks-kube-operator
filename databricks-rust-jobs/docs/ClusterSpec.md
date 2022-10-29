# ClusterSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**existing_cluster_id** | Option<**String**> | If existing_cluster_id, the ID of an existing cluster that is used for all runs of this job. When running jobs on an existing cluster, you may need to manually restart the cluster if it stops responding. We suggest running jobs on new clusters for greater reliability. | [optional]
**new_cluster** | Option<[**crate::models::NewCluster**](NewCluster.md)> |  | [optional]
**libraries** | Option<[**Vec<crate::models::Library>**](Library.md)> | An optional list of libraries to be installed on the cluster that executes the job. The default value is an empty list. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


