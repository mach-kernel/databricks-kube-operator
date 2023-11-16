# \KeyConfigurationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**encryption_keyscreate**](KeyConfigurationsApi.md#encryption_keyscreate) | **POST** /api/2.0/accounts/{account_id}/customer-managed-keys | Create encryption key configuration
[**encryption_keysdelete**](KeyConfigurationsApi.md#encryption_keysdelete) | **DELETE** /api/2.0/accounts/{account_id}/customer-managed-keys/{customer_managed_key_id} | Delete encryption key configuration
[**encryption_keysget**](KeyConfigurationsApi.md#encryption_keysget) | **GET** /api/2.0/accounts/{account_id}/customer-managed-keys/{customer_managed_key_id} | Get encryption key configuration
[**encryption_keyslist**](KeyConfigurationsApi.md#encryption_keyslist) | **GET** /api/2.0/accounts/{account_id}/customer-managed-keys | Get all encryption key configurations



## encryption_keyscreate

> crate::models::ProvisioningCustomerManagedKey encryption_keyscreate(account_id, provisioning_create_customer_managed_key_request)
Create encryption key configuration

Creates a customer-managed key configuration object for an account, specified by ID. This operation uploads a reference to a customer-managed key to Databricks. If the key is assigned as a workspace's customer-managed key for managed services, Databricks uses the key to encrypt the workspaces notebooks and secrets in the control plane, in addition to Databricks SQL queries and query history. If it is specified as a workspace's customer-managed key for workspace storage, the key encrypts the workspace's root S3 bucket (which contains the workspace's root DBFS and system data) and, optionally, cluster EBS volume data.  **Important**: Customer-managed keys are supported only for some deployment types, subscription types, and AWS regions that currently support creation of Databricks workspaces.  This operation is available only if your account is on the E2 version of the platform or on a select custom plan that allows multiple workspaces per account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**provisioning_create_customer_managed_key_request** | [**ProvisioningCreateCustomerManagedKeyRequest**](ProvisioningCreateCustomerManagedKeyRequest.md) | Properties of the encryption key configuration. | [required] |

### Return type

[**crate::models::ProvisioningCustomerManagedKey**](ProvisioningCustomerManagedKey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## encryption_keysdelete

> serde_json::Value encryption_keysdelete(account_id, customer_managed_key_id)
Delete encryption key configuration

Deletes a customer-managed key configuration object for an account. You cannot delete a configuration that is associated with a running workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**customer_managed_key_id** | [**serde_json::Value**](.md) | Databricks encryption key configuration ID. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## encryption_keysget

> crate::models::ProvisioningCustomerManagedKey encryption_keysget(account_id, customer_managed_key_id)
Get encryption key configuration

Gets a customer-managed key configuration object for an account, specified by ID. This operation uploads a reference to a customer-managed key to Databricks. If assigned as a workspace's customer-managed key for managed services, Databricks uses the key to encrypt the workspaces notebooks and secrets in the control plane, in addition to Databricks SQL queries and query history. If it is specified as a workspace's customer-managed key for storage, the key encrypts the workspace's root S3 bucket (which contains the workspace's root DBFS and system data) and, optionally, cluster EBS volume data.  **Important**: Customer-managed keys are supported only for some deployment types, subscription types, and AWS regions.  This operation is available only if your account is on the E2 version of the platform.\", 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**customer_managed_key_id** | [**serde_json::Value**](.md) | Databricks encryption key configuration ID. | [required] |

### Return type

[**crate::models::ProvisioningCustomerManagedKey**](ProvisioningCustomerManagedKey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## encryption_keyslist

> Vec<crate::models::ProvisioningCustomerManagedKey> encryption_keyslist(account_id)
Get all encryption key configurations

Gets all customer-managed key configuration objects for an account. If the key is specified as a workspace's managed services customer-managed key, Databricks uses the key to encrypt the workspace's notebooks and secrets in the control plane, in addition to Databricks SQL queries and query history. If the key is specified as a workspace's storage customer-managed key, the key is used to encrypt the workspace's root S3 bucket and optionally can encrypt cluster EBS volumes data in the data plane.  **Important**: Customer-managed keys are supported only for some deployment types, subscription types, and AWS regions.  This operation is available only if your account is on the E2 version of the platform. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |

### Return type

[**Vec<crate::models::ProvisioningCustomerManagedKey>**](ProvisioningCustomerManagedKey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

