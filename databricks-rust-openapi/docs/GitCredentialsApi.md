# \GitCredentialsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**git_credentialscreate**](GitCredentialsApi.md#git_credentialscreate) | **POST** /api/2.0/git-credentials | Create a credential entry
[**git_credentialsdelete**](GitCredentialsApi.md#git_credentialsdelete) | **DELETE** /api/2.0/git-credentials/{credential_id} | Delete a credential
[**git_credentialsget**](GitCredentialsApi.md#git_credentialsget) | **GET** /api/2.0/git-credentials/{credential_id} | Get a credential entry
[**git_credentialslist**](GitCredentialsApi.md#git_credentialslist) | **GET** /api/2.0/git-credentials | Get Git credentials
[**git_credentialsupdate**](GitCredentialsApi.md#git_credentialsupdate) | **PATCH** /api/2.0/git-credentials/{credential_id} | Update a credential



## git_credentialscreate

> crate::models::WorkspaceCreateCredentialsResponse git_credentialscreate(workspace_create_credentials)
Create a credential entry

Creates a Git credential entry for the user. Only one Git credential per user is  supported, so any attempts to create credentials if an entry already exists will  fail. Use the PATCH endpoint to update existing credentials, or the DELETE endpoint to  delete existing credentials. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_create_credentials** | [**WorkspaceCreateCredentials**](WorkspaceCreateCredentials.md) |  | [required] |

### Return type

[**crate::models::WorkspaceCreateCredentialsResponse**](WorkspaceCreateCredentialsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## git_credentialsdelete

> git_credentialsdelete(credential_id)
Delete a credential

Deletes the specified Git credential.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **i64** | The ID for the corresponding credential to access. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## git_credentialsget

> crate::models::WorkspaceCredentialInfo git_credentialsget(credential_id)
Get a credential entry

Gets the Git credential with the specified credential ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **i64** | The ID for the corresponding credential to access. | [required] |

### Return type

[**crate::models::WorkspaceCredentialInfo**](WorkspaceCredentialInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## git_credentialslist

> crate::models::WorkspaceGetCredentialsResponse git_credentialslist()
Get Git credentials

Lists the calling user's Git credentials. One credential per user is supported.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WorkspaceGetCredentialsResponse**](WorkspaceGetCredentialsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## git_credentialsupdate

> serde_json::Value git_credentialsupdate(credential_id, workspace_update_credentials)
Update a credential

Updates the specified Git credential.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **i64** | The ID for the corresponding credential to access. | [required] |
**workspace_update_credentials** | [**WorkspaceUpdateCredentials**](WorkspaceUpdateCredentials.md) | Details required to update the credential | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

