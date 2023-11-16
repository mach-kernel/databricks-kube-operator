# ProvisioningCreateWorkspaceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cloud_resource_container** | Option<[**crate::models::ProvisioningCloudResourceContainer**](ProvisioningCloudResourceContainer.md)> |  | [optional]
**private_access_settings_id** | Option<[**String**](String.md)> | ID of the workspace's private access settings object. Only used for PrivateLink. This ID must be specified for customers using [AWS PrivateLink](https://Awsamazon.com/privatelink/) for either front-end (user-to-workspace connection), back-end (data plane to control plane connection), or both connection types.  Before configuring PrivateLink, read the [Databricks article about PrivateLink](https://docs.databricks.com/administration-guide/cloud-configurations/aws/privatelink.html). | [optional]
**cloud** | Option<**String**> | The cloud provider which the workspace uses. For Google Cloud workspaces, always set this field to `gcp`. | [optional]
**storage_configuration_id** | Option<[**String**](String.md)> | The ID of the workspace's storage configuration object. | [optional]
**managed_services_customer_managed_key_id** | Option<[**String**](String.md)> | The ID of the workspace's managed services encryption key configuration object. This is used to help protect and control access to the workspace's notebooks, secrets, Databricks SQL queries, and query history. The provided key configuration object property `use_cases` must contain `MANAGED_SERVICES`. | [optional]
**aws_region** | Option<**String**> | The AWS region of the workspace's data plane. | [optional]
**gcp_managed_network_config** | Option<[**crate::models::ProvisioningGcpManagedNetworkConfig**](ProvisioningGcpManagedNetworkConfig.md)> |  | [optional]
**storage_customer_managed_key_id** | Option<[**String**](String.md)> | The ID of the workspace's storage encryption key configuration object. This is used to encrypt the workspace's root S3 bucket (root DBFS and system data) and, optionally, cluster EBS volumes. The provided key configuration object property `use_cases` must contain `STORAGE`. | [optional]
**location** | Option<**String**> | The Google Cloud region of the workspace data plane in your Google account. For example, `us-east4`. | [optional]
**network_id** | Option<[**String**](String.md)> |  | [optional]
**credentials_id** | Option<[**String**](String.md)> | ID of the workspace's credential configuration object. | [optional]
**deployment_name** | Option<**String**> | The deployment name defines part of the subdomain for the workspace. The workspace URL for web application and REST APIs is `<workspace-deployment-name>.Clouddatabricks.com`. For example, if the deployment name is `abcsales`, your workspace URL will be `https://abcsales.Clouddatabricks.com`. Hyphens are allowed.  This property supports only the set of characters that are allowed in a subdomain.  If your account has a non-empty deployment name prefix at workspace creation time, the workspace deployment name changes so that the beginning has the account prefix and a hyphen. For example, if your account's deployment prefix is `acme` and the workspace deployment name is `workspace-1`, the `deployment_name` field becomes `acme-workspace-1` and that is the value that is returned in JSON responses for the `deployment_name` field. The workspace URL is `acme-workspace-1.Clouddatabricks.com`.  If your account has a non-empty deployment name prefix and you set `deployment_name` to the reserved keyword `EMPTY`, `deployment_name` is just the account prefix only. For example, if your account's deployment prefix is `acme` and the workspace deployment name is `EMPTY`, `deployment_name` becomes `acme` only and the workspace URL is `acme.Clouddatabricks.com`.  Contact your Databricks representatives to add an account deployment name prefix to your account. If you do not have a deployment name prefix, the special deployment name value `EMPTY` is invalid.  This value must be unique across all non-deleted deployments across all AWS regions.  If a new workspace omits this property, the server generates a unique deployment name for you with the pattern `dbc-xxxxxxxx-xxxx`. | [optional]
**pricing_tier** | Option<[**crate::models::ProvisioningPricingTier**](ProvisioningPricingTier.md)> |  | [optional]
**gke_config** | Option<[**crate::models::ProvisioningGkeConfig**](ProvisioningGkeConfig.md)> |  | [optional]
**workspace_name** | **String** | The workspace's human-readable name. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


