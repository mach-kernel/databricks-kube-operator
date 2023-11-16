# \StorageConfigurationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**storagecreate**](StorageConfigurationsApi.md#storagecreate) | **POST** /api/2.0/accounts/{account_id}/storage-configurations | Create new storage configuration
[**storagedelete**](StorageConfigurationsApi.md#storagedelete) | **DELETE** /api/2.0/accounts/{account_id}/storage-configurations/{storage_configuration_id} | Delete storage configuration
[**storageget**](StorageConfigurationsApi.md#storageget) | **GET** /api/2.0/accounts/{account_id}/storage-configurations/{storage_configuration_id} | Get storage configuration
[**storagelist**](StorageConfigurationsApi.md#storagelist) | **GET** /api/2.0/accounts/{account_id}/storage-configurations | Get all storage configurations



## storagecreate

> crate::models::ProvisioningStorageConfiguration storagecreate(account_id, provisioning_create_storage_configuration_request)
Create new storage configuration

Creates new storage configuration for an account, specified by ID. Uploads a storage configuration object that represents the root AWS S3 bucket in your account. Databricks stores related workspace assets including DBFS, cluster logs, and job results. For the AWS S3 bucket, you need to configure the required bucket policy.  For information about how to create a new workspace with this API, see [Create a new workspace using the Account API](http://Docsdatabricks.com/administration-guide/account-api/new-workspace.html)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**provisioning_create_storage_configuration_request** | [**ProvisioningCreateStorageConfigurationRequest**](ProvisioningCreateStorageConfigurationRequest.md) | Properties of the new storage configuration. | [required] |

### Return type

[**crate::models::ProvisioningStorageConfiguration**](ProvisioningStorageConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storagedelete

> serde_json::Value storagedelete(account_id, storage_configuration_id)
Delete storage configuration

Deletes a Databricks storage configuration. You cannot delete a storage configuration that is associated with any workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**storage_configuration_id** | [**serde_json::Value**](.md) | Databricks Account API storage configuration ID. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storageget

> crate::models::ProvisioningStorageConfiguration storageget(account_id, storage_configuration_id)
Get storage configuration

Gets a Databricks storage configuration for an account, both specified by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**storage_configuration_id** | [**serde_json::Value**](.md) | Databricks Account API storage configuration ID. | [required] |

### Return type

[**crate::models::ProvisioningStorageConfiguration**](ProvisioningStorageConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storagelist

> Vec<crate::models::ProvisioningStorageConfiguration> storagelist(account_id)
Get all storage configurations

Gets a list of all Databricks storage configurations for your account, specified by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |

### Return type

[**Vec<crate::models::ProvisioningStorageConfiguration>**](ProvisioningStorageConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

