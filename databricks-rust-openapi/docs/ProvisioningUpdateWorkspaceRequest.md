# ProvisioningUpdateWorkspaceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aws_region** | Option<**String**> | The AWS region of the workspace's data plane (for example, `us-west-2`). This parameter is available only for updating failed workspaces. | [optional]
**credentials_id** | Option<[**String**](String.md)> | ID of the workspace's credential configuration object. This parameter is available for updating both failed and running workspaces. | [optional]
**managed_services_customer_managed_key_id** | Option<[**String**](String.md)> | The ID of the workspace's managed services encryption key configuration object. This parameter is available only for updating failed workspaces. | [optional]
**network_id** | Option<[**String**](String.md)> | The ID of the workspace's network configuration object. Used only if you already use a customer-managed VPC. For failed workspaces only, you can switch from a Databricks-managed VPC to a customer-managed VPC by updating the workspace to add a network configuration ID. | [optional]
**storage_configuration_id** | Option<[**String**](String.md)> | The ID of the workspace's storage configuration object. This parameter is available only for updating failed workspaces. | [optional]
**storage_customer_managed_key_id** | Option<[**String**](String.md)> | The ID of the key configuration object for workspace storage. This parameter is available for updating both failed and running workspaces. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


