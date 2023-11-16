# \CredentialConfigurationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**credentialscreate**](CredentialConfigurationsApi.md#credentialscreate) | **POST** /api/2.0/accounts/{account_id}/credentials | Create credential configuration
[**credentialsdelete**](CredentialConfigurationsApi.md#credentialsdelete) | **DELETE** /api/2.0/accounts/{account_id}/credentials/{credentials_id} | Delete credential configuration
[**credentialsget**](CredentialConfigurationsApi.md#credentialsget) | **GET** /api/2.0/accounts/{account_id}/credentials/{credentials_id} | Get credential configuration
[**credentialslist**](CredentialConfigurationsApi.md#credentialslist) | **GET** /api/2.0/accounts/{account_id}/credentials | Get all credential configurations



## credentialscreate

> crate::models::ProvisioningCredential credentialscreate(account_id, provisioning_create_credential_request)
Create credential configuration

Creates a Databricks credential configuration that represents cloud cross-account credentials for a specified account. Databricks uses this to set up network infrastructure properly to host Databricks clusters. For your AWS IAM role, you need to trust the External ID (the Databricks Account API account ID)  in the returned credential object, and configure the required access policy.  Save the response's `credentials_id` field, which is the ID for your new credential configuration object.  For information about how to create a new workspace with this API, see [Create a new workspace using the Account API](http://Docsdatabricks.com/administration-guide/account-api/new-workspace.html)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**provisioning_create_credential_request** | [**ProvisioningCreateCredentialRequest**](ProvisioningCreateCredentialRequest.md) | Properties of the new credential configuration. | [required] |

### Return type

[**crate::models::ProvisioningCredential**](ProvisioningCredential.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## credentialsdelete

> serde_json::Value credentialsdelete(account_id, credentials_id)
Delete credential configuration

Deletes a Databricks credential configuration object for an account, both specified by ID. You cannot delete a credential that is associated with any workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**credentials_id** | [**serde_json::Value**](.md) | Databricks Account API credential configuration ID | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## credentialsget

> crate::models::ProvisioningCredential credentialsget(account_id, credentials_id)
Get credential configuration

Gets a Databricks credential configuration object for an account, both specified by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**credentials_id** | [**serde_json::Value**](.md) | Databricks Account API credential configuration ID | [required] |

### Return type

[**crate::models::ProvisioningCredential**](ProvisioningCredential.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## credentialslist

> Vec<crate::models::ProvisioningCredential> credentialslist(account_id)
Get all credential configurations

Gets all Databricks credential configurations associated with an account specified by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |

### Return type

[**Vec<crate::models::ProvisioningCredential>**](ProvisioningCredential.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

