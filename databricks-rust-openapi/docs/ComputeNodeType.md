# ComputeNodeType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**support_cluster_tags** | Option<**bool**> |  | [optional]
**support_ebs_volumes** | Option<**bool**> |  | [optional]
**instance_type_id** | **String** | An identifier for the type of hardware that this node runs on, Eg., \"r3.2xlarge\" in AWS. | 
**memory_mb** | **i32** | Memory (in MB) available for this node type. | 
**is_graviton** | Option<**bool**> |  | [optional]
**is_io_cache_enabled** | Option<**bool**> |  | [optional]
**node_instance_type** | Option<[**crate::models::ComputeNodeInstanceType**](ComputeNodeInstanceType.md)> |  | [optional]
**node_info** | Option<[**crate::models::ComputeCloudProviderNodeInfo**](ComputeCloudProviderNodeInfo.md)> |  | [optional]
**is_deprecated** | Option<**bool**> | Whether the node type is deprecated. Non-deprecated node types offer greater performance. | [optional][default to false]
**is_encrypted_in_transit** | Option<**bool**> | AWS specific, whether this instance supports encryption in transit, used for hipaa and pci workloads. | [optional][default to false]
**is_hidden** | Option<**bool**> |  | [optional]
**support_port_forwarding** | Option<**bool**> |  | [optional]
**photon_driver_capable** | Option<**bool**> |  | [optional]
**category** | Option<**String**> |  | [optional]
**node_type_id** | **String** | Unique identifier for this node type. | 
**num_cores** | **f64** | Number of CPU cores available for this node type. Note that this can be fractional, Eg., 2.5 cores, if the the number of cores on a machine instance is not divisible by the number of Spark nodes on that machine. | 
**description** | **String** | A string description associated with this node type, Eg., \"r3.xlarge\". | 
**num_gpus** | Option<**i32**> |  | [optional]
**display_order** | Option<**i32**> |  | [optional]
**photon_worker_capable** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


