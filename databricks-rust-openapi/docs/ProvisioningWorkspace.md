# ProvisioningWorkspace

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cloud_resource_container** | Option<[**crate::models::ProvisioningCloudResourceContainer**](ProvisioningCloudResourceContainer.md)> |  | [optional]
**private_access_settings_id** | Option<[**String**](String.md)> | ID of the workspace's private access settings object. Only used for PrivateLink. You must specify this ID if you are using [AWS PrivateLink](https://Awsamazon.com/privatelink/) for either front-end (user-to-workspace connection), back-end (data plane to control plane connection), or both connection types.  Before configuring PrivateLink, read the [Databricks article about PrivateLink](https://docs.databricks.com/administration-guide/cloud-configurations/aws/privatelink.html). | [optional]
**cloud** | Option<**String**> | The cloud name. This field always has the value `gcp`. | [optional]
**storage_configuration_id** | Option<[**String**](String.md)> | ID of the workspace's storage configuration object. | [optional]
**creation_time** | Option<**i64**> | Time in epoch milliseconds when the workspace was created. | [optional]
**managed_services_customer_managed_key_id** | Option<[**String**](String.md)> | ID of the key configuration for encrypting managed services. | [optional]
**aws_region** | Option<**String**> | The AWS region of the workspace data plane (for example, `us-west-2`). | [optional]
**gcp_managed_network_config** | Option<[**crate::models::ProvisioningGcpManagedNetworkConfig**](ProvisioningGcpManagedNetworkConfig.md)> |  | [optional]
**storage_customer_managed_key_id** | Option<[**String**](String.md)> | ID of the key configuration for encrypting workspace storage. | [optional]
**location** | Option<**String**> | The Google Cloud region of the workspace data plane in your Google account (for example, `us-east4`). | [optional]
**workspace_id** | Option<**i64**> | A unique integer ID for the workspace | [optional]
**network_id** | Option<[**String**](String.md)> | The network configuration ID that is attached to the workspace. This field is available only if the network is a customer-managed network. | [optional]
**workspace_status_message** | Option<**String**> | Message describing the current workspace status. | [optional]
**credentials_id** | Option<[**String**](String.md)> | ID of the workspace's credential configuration object. | [optional]
**deployment_name** | Option<**String**> | The deployment name defines part of the subdomain for the workspace. The workspace URL for web application and REST APIs is `<deployment-name>.Clouddatabricks.com`.  This value must be unique across all non-deleted deployments across all AWS regions. | [optional]
**pricing_tier** | Option<[**crate::models::ProvisioningPricingTier**](ProvisioningPricingTier.md)> |  | [optional]
**account_id** | Option<[**String**](String.md)> | Databricks account ID. | [optional]
**gke_config** | Option<[**crate::models::ProvisioningGkeConfig**](ProvisioningGkeConfig.md)> |  | [optional]
**workspace_status** | Option<[**crate::models::ProvisioningWorkspaceStatus**](ProvisioningWorkspaceStatus.md)> |  | [optional]
**workspace_name** | Option<**String**> | The human-readable name of the workspace. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


