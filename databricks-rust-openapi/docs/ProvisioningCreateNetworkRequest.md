# ProvisioningCreateNetworkRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**gcp_network_info** | Option<[**crate::models::ProvisioningGcpNetworkInfo**](ProvisioningGcpNetworkInfo.md)> |  | [optional]
**network_name** | **String** | The human-readable name of the network configuration. | 
**security_group_ids** | Option<**Vec<String>**> |  | [optional]
**subnet_ids** | Option<**Vec<String>**> |  | [optional]
**vpc_endpoints** | Option<[**crate::models::ProvisioningNetworkVpcEndpoints**](ProvisioningNetworkVpcEndpoints.md)> |  | [optional]
**vpc_id** | Option<**String**> | The ID of the VPC associated with this network. VPC IDs can be used in multiple network configurations. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


