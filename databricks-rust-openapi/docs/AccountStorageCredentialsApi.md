# \AccountStorageCredentialsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_storage_credentialscreate**](AccountStorageCredentialsApi.md#account_storage_credentialscreate) | **POST** /api/2.0/accounts/{account_id}/metastores/{metastore_id}/storage-credentials | Create a storage credential
[**account_storage_credentialsdelete**](AccountStorageCredentialsApi.md#account_storage_credentialsdelete) | **DELETE** /api/2.0/accounts/{account_id}/metastores/{metastore_id}/storage-credentials/{storage_credential_name} | Delete a storage credential
[**account_storage_credentialsget**](AccountStorageCredentialsApi.md#account_storage_credentialsget) | **GET** /api/2.0/accounts/{account_id}/metastores/{metastore_id}/storage-credentials/{storage_credential_name} | Gets the named storage credential
[**account_storage_credentialslist**](AccountStorageCredentialsApi.md#account_storage_credentialslist) | **GET** /api/2.0/accounts/{account_id}/metastores/{metastore_id}/storage-credentials | Get all storage credentials assigned to a metastore
[**account_storage_credentialsupdate**](AccountStorageCredentialsApi.md#account_storage_credentialsupdate) | **PUT** /api/2.0/accounts/{account_id}/metastores/{metastore_id}/storage-credentials/{storage_credential_name} | Updates a storage credential



## account_storage_credentialscreate

> crate::models::CatalogAccountsStorageCredentialInfo account_storage_credentialscreate(account_id, metastore_id, catalog_accounts_create_storage_credential)
Create a storage credential

Creates a new storage credential. The request object is specific to the cloud:    * **AwsIamRole** for AWS credentials   * **AzureServicePrincipal** for Azure credentials   * **GcpServiceAcountKey** for GCP credentials.  The caller must be a metastore admin and have the **CREATE_STORAGE_CREDENTIAL** privilege on the metastore. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**metastore_id** | [**serde_json::Value**](.md) | Unity Catalog metastore ID | [required] |
**catalog_accounts_create_storage_credential** | Option<[**CatalogAccountsCreateStorageCredential**](CatalogAccountsCreateStorageCredential.md)> |  |  |

### Return type

[**crate::models::CatalogAccountsStorageCredentialInfo**](CatalogAccountsStorageCredentialInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_storage_credentialsdelete

> serde_json::Value account_storage_credentialsdelete(account_id, metastore_id, name, force)
Delete a storage credential

Deletes a storage credential from the metastore. The caller must be an owner of the storage credential.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**metastore_id** | [**serde_json::Value**](.md) | Unity Catalog metastore ID | [required] |
**name** | [**serde_json::Value**](.md) | Name of the storage credential. | [required] |
**force** | Option<**bool**> | Force deletion even if the Storage Credential is not empty. Default is false. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_storage_credentialsget

> crate::models::CatalogAccountsStorageCredentialInfo account_storage_credentialsget(account_id, metastore_id, name)
Gets the named storage credential

Gets a storage credential from the metastore. The caller must be a metastore admin, the owner of the storage credential, or have a level of privilege on the storage credential. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**metastore_id** | [**serde_json::Value**](.md) | Unity Catalog metastore ID | [required] |
**name** | [**serde_json::Value**](.md) | Name of the storage credential. | [required] |

### Return type

[**crate::models::CatalogAccountsStorageCredentialInfo**](CatalogAccountsStorageCredentialInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_storage_credentialslist

> crate::models::CatalogListStorageCredentialsResponse account_storage_credentialslist(account_id, metastore_id)
Get all storage credentials assigned to a metastore

Gets a list of all storage credentials that have been assigned to given metastore.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**metastore_id** | [**serde_json::Value**](.md) | Unity Catalog metastore ID | [required] |

### Return type

[**crate::models::CatalogListStorageCredentialsResponse**](CatalogListStorageCredentialsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_storage_credentialsupdate

> crate::models::CatalogAccountsStorageCredentialInfo account_storage_credentialsupdate(account_id, metastore_id, name, catalog_accounts_update_storage_credential)
Updates a storage credential

Updates a storage credential on the metastore. The caller must be the owner of the storage credential. If the caller is a metastore admin, only the __owner__ credential can be changed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**metastore_id** | [**serde_json::Value**](.md) | Unity Catalog metastore ID | [required] |
**name** | [**serde_json::Value**](.md) | Name of the storage credential. | [required] |
**catalog_accounts_update_storage_credential** | [**CatalogAccountsUpdateStorageCredential**](CatalogAccountsUpdateStorageCredential.md) | The storage credential to update. | [required] |

### Return type

[**crate::models::CatalogAccountsStorageCredentialInfo**](CatalogAccountsStorageCredentialInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

