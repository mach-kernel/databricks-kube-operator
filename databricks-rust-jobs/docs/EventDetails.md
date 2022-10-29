# EventDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_num_workers** | Option<**i32**> | The number of nodes in the cluster. | [optional]
**target_num_workers** | Option<**i32**> | The targeted number of nodes in the cluster. | [optional]
**previous_attributes** | Option<[**crate::models::AwsAttributes**](AwsAttributes.md)> |  | [optional]
**attributes** | Option<[**crate::models::AwsAttributes**](AwsAttributes.md)> |  | [optional]
**previous_cluster_size** | Option<[**crate::models::ClusterSize**](ClusterSize.md)> |  | [optional]
**cluster_size** | Option<[**crate::models::ClusterSize**](ClusterSize.md)> |  | [optional]
**cause** | Option<[**crate::models::ResizeCause**](ResizeCause.md)> |  | [optional]
**reason** | Option<[**crate::models::TerminationReason**](TerminationReason.md)> |  | [optional]
**user** | Option<**String**> | The user that caused the event to occur. (Empty if it was done by Databricks.) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


