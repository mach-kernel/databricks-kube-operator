# ComputeResizeCluster

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**autoscale** | Option<[**crate::models::ComputeAutoScale**](ComputeAutoScale.md)> | Parameters needed in order to automatically scale clusters up and down based on load. Note: autoscaling works best with DB runtime versions 3.0 or later. | [optional]
**cluster_id** | **String** | The cluster to be resized. | 
**num_workers** | Option<**i32**> | Number of worker nodes that this cluster should have. A cluster has one Spark Driver and `num_workers` Executors for a total of `num_workers` + 1 Spark nodes.  Note: When reading the properties of a cluster, this field reflects the desired number of workers rather than the actual current number of workers. For instance, if a cluster is resized from 5 to 10 workers, this field will immediately be updated to reflect the target size of 10 workers, whereas the workers listed in `spark_info` will gradually increase from 5 to 10 as the new nodes are provisioned. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


