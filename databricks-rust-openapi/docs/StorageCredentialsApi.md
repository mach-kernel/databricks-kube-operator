# \StorageCredentialsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**storage_credentialscreate**](StorageCredentialsApi.md#storage_credentialscreate) | **POST** /api/2.1/unity-catalog/storage-credentials | Create a storage credential
[**storage_credentialsdelete**](StorageCredentialsApi.md#storage_credentialsdelete) | **DELETE** /api/2.1/unity-catalog/storage-credentials/{name} | Delete a credential
[**storage_credentialsget**](StorageCredentialsApi.md#storage_credentialsget) | **GET** /api/2.1/unity-catalog/storage-credentials/{name} | Get a credential
[**storage_credentialslist**](StorageCredentialsApi.md#storage_credentialslist) | **GET** /api/2.1/unity-catalog/storage-credentials | List credentials
[**storage_credentialsupdate**](StorageCredentialsApi.md#storage_credentialsupdate) | **PATCH** /api/2.1/unity-catalog/storage-credentials/{name} | Update a credential
[**storage_credentialsvalidate**](StorageCredentialsApi.md#storage_credentialsvalidate) | **POST** /api/2.1/unity-catalog/validate-storage-credentials | Validate a storage credential



## storage_credentialscreate

> crate::models::CatalogStorageCredentialInfo storage_credentialscreate(catalog_create_storage_credential)
Create a storage credential

Creates a new storage credential. The request object is specific to the cloud:    * **AwsIamRole** for AWS credentials.   * **AzureServicePrincipal** for Azure credentials.   * **AzureManagedIdentity** for Azure managed credentials.   * **DatabricksGcpServiceAccount** for GCP managed credentials.  The caller must be a metastore admin and have the **CREATE_STORAGE_CREDENTIAL** privilege on the metastore. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_create_storage_credential** | Option<[**CatalogCreateStorageCredential**](CatalogCreateStorageCredential.md)> |  |  |

### Return type

[**crate::models::CatalogStorageCredentialInfo**](CatalogStorageCredentialInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_credentialsdelete

> serde_json::Value storage_credentialsdelete(name, force)
Delete a credential

Deletes a storage credential from the metastore. The caller must be an owner of the storage credential.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**serde_json::Value**](.md) | Name of the storage credential. | [required] |
**force** | Option<**bool**> | Force deletion even if there are dependent external locations or external tables. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_credentialsget

> crate::models::CatalogStorageCredentialInfo storage_credentialsget(name)
Get a credential

Gets a storage credential from the metastore. The caller must be a metastore admin, the owner of the storage credential, or have some permission on the storage credential. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**serde_json::Value**](.md) | Name of the storage credential. | [required] |

### Return type

[**crate::models::CatalogStorageCredentialInfo**](CatalogStorageCredentialInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_credentialslist

> crate::models::CatalogListStorageCredentialsResponse storage_credentialslist()
List credentials

Gets an array of storage credentials (as __StorageCredentialInfo__ objects). The array is limited to only those storage credentials the caller has permission to access. If the caller is a metastore admin, all storage credentials will be retrieved. There is no guarantee of a specific ordering of the elements in the array. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CatalogListStorageCredentialsResponse**](CatalogListStorageCredentialsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_credentialsupdate

> crate::models::CatalogStorageCredentialInfo storage_credentialsupdate(name, catalog_update_storage_credential)
Update a credential

Updates a storage credential on the metastore. The caller must be the owner of the storage credential or a metastore admin. If the caller is a metastore admin, only the __owner__ credential can be changed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**serde_json::Value**](.md) | Name of the storage credential. | [required] |
**catalog_update_storage_credential** | Option<[**CatalogUpdateStorageCredential**](CatalogUpdateStorageCredential.md)> |  |  |

### Return type

[**crate::models::CatalogStorageCredentialInfo**](CatalogStorageCredentialInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## storage_credentialsvalidate

> crate::models::CatalogValidateStorageCredentialResponse storage_credentialsvalidate(catalog_validate_storage_credential)
Validate a storage credential

Validates a storage credential. At least one of __external_location_name__ and __url__ need to be provided. If only one of them is provided, it will be used for validation. And if both are provided, the __url__ will be used for validation, and __external_location_name__ will be ignored when checking overlapping urls.  Either the __storage_credential_name__ or the cloud-specific credential must be provided.  The caller must be a metastore admin or the storage credential owner or have the **CREATE_EXTERNAL_LOCATION** privilege on the metastore and the storage credential. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_validate_storage_credential** | Option<[**CatalogValidateStorageCredential**](CatalogValidateStorageCredential.md)> |  |  |

### Return type

[**crate::models::CatalogValidateStorageCredentialResponse**](CatalogValidateStorageCredentialResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

