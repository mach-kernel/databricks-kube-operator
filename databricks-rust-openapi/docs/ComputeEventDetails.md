# ComputeEventDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enable_termination_for_node_blocklisted** | Option<**bool**> | Whether or not a blocklisted node should be terminated. For ClusterEventType NODE_BLACKLISTED. | [optional]
**user** | Option<**String**> | The user that caused the event to occur. (Empty if it was done by the control plane.) | [optional]
**free_space** | Option<**i64**> | <needs content added> | [optional]
**previous_cluster_size** | Option<[**crate::models::ComputeClusterSize**](ComputeClusterSize.md)> | The size of the cluster before an edit or resize. | [optional]
**instance_id** | Option<**String**> | Instance Id where the event originated from | [optional]
**cause** | Option<**String**> | The cause of a change in target size. | [optional]
**current_num_workers** | Option<**i32**> | The current number of nodes in the cluster. | [optional]
**disk_size** | Option<**i64**> | Current disk size in bytes | [optional]
**target_num_workers** | Option<**i32**> | The targeted number of nodes in the cluster. | [optional]
**attributes** | Option<[**crate::models::ComputeClusterAttributes**](ComputeClusterAttributes.md)> | * For created clusters, the attributes of the cluster. * For edited clusters, the new attributes of the cluster. | [optional]
**previous_disk_size** | Option<**i64**> | Previous disk size in bytes | [optional]
**cluster_size** | Option<[**crate::models::ComputeClusterSize**](ComputeClusterSize.md)> | The actual cluster size that was set in the cluster creation or edit. | [optional]
**driver_state_message** | Option<**String**> | More details about the change in driver's state | [optional]
**job_run_name** | Option<**String**> | Unique identifier of the specific job run associated with this cluster event * For clusters created for jobs, this will be the same as the cluster name | [optional]
**reason** | Option<[**crate::models::ComputeTerminationReason**](ComputeTerminationReason.md)> | A termination reason:   * On a TERMINATED event, this is the reason of the termination.   * On a RESIZE_COMPLETE event, this indicates the reason that we failed to acquire some nodes. | [optional]
**target_num_vcpus** | Option<**i32**> | The targeted number of vCPUs in the cluster. | [optional]
**current_num_vcpus** | Option<**i32**> | The current number of vCPUs in the cluster. | [optional]
**previous_attributes** | Option<[**crate::models::ComputeClusterAttributes**](ComputeClusterAttributes.md)> | The cluster attributes before a cluster was edited. | [optional]
**did_not_expand_reason** | Option<**String**> | <needs content added> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


