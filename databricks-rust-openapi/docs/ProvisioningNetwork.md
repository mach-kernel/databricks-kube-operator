# ProvisioningNetwork

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**creation_time** | Option<**i64**> | Time in epoch milliseconds when the network was created. | [optional]
**vpc_endpoints** | Option<[**crate::models::ProvisioningNetworkVpcEndpoints**](ProvisioningNetworkVpcEndpoints.md)> |  | [optional]
**security_group_ids** | Option<**Vec<String>**> |  | [optional]
**warning_messages** | Option<[**Vec<crate::models::ProvisioningNetworkWarning>**](ProvisioningNetworkWarning.md)> |  | [optional]
**network_name** | Option<**String**> | The human-readable name of the network configuration. | [optional]
**workspace_id** | Option<**i64**> | Workspace ID associated with this network configuration. | [optional]
**network_id** | Option<[**String**](String.md)> | The Databricks network configuration ID. | [optional]
**vpc_status** | Option<[**crate::models::ProvisioningVpcStatus**](ProvisioningVpcStatus.md)> |  | [optional]
**gcp_network_info** | Option<[**crate::models::ProvisioningGcpNetworkInfo**](ProvisioningGcpNetworkInfo.md)> |  | [optional]
**subnet_ids** | Option<**Vec<String>**> |  | [optional]
**error_messages** | Option<[**Vec<crate::models::ProvisioningNetworkHealth>**](ProvisioningNetworkHealth.md)> |  | [optional]
**vpc_id** | Option<**String**> | The ID of the VPC associated with this network configuration. VPC IDs can be used in multiple networks. | [optional]
**account_id** | Option<[**String**](String.md)> | The Databricks account ID associated with this network configuration. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


