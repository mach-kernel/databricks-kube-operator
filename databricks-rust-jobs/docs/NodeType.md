# NodeType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_type_id** | **String** | Unique identifier for this node type. This field is required. | 
**memory_mb** | **i32** | Memory (in MB) available for this node type. This field is required. | 
**num_cores** | Option<**f32**> | Number of CPU cores available for this node type. This can be fractional if the number of cores on a machine instance is not divisible by the number of Spark nodes on that machine. This field is required. | [optional]
**description** | **String** | A string description associated with this node type. This field is required. | 
**instance_type_id** | **String** | An identifier for the type of hardware that this node runs on. This field is required. | 
**is_deprecated** | Option<**bool**> | Whether the node type is deprecated. Non-deprecated node types offer greater performance. | [optional]
**node_info** | Option<[**crate::models::ClusterCloudProviderNodeInfo**](ClusterCloudProviderNodeInfo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


