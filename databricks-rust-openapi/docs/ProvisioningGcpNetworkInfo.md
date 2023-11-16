# ProvisioningGcpNetworkInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**network_project_id** | **String** | The Google Cloud project ID of the VPC network. | 
**pod_ip_range_name** | **String** | The name of the secondary IP range for pods. A Databricks-managed GKE cluster uses this IP range for its pods. This secondary IP range can be used by only one workspace. | 
**service_ip_range_name** | **String** | The name of the secondary IP range for services. A Databricks-managed GKE cluster uses this IP range for its services. This secondary IP range can be used by only one workspace. | 
**subnet_id** | **String** | The ID of the subnet associated with this network. | 
**subnet_region** | **String** | The Google Cloud region of the workspace data plane (for example, `us-east4`). | 
**vpc_id** | **String** | The ID of the VPC associated with this network. VPC IDs can be used in multiple network configurations. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


