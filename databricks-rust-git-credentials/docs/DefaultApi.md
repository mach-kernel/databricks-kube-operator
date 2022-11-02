# \DefaultApi

All URIs are relative to *https://<databricks-instance>/api/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_git_credential**](DefaultApi.md#create_git_credential) | **POST** /git-credentials | Create a Git credential entry
[**delete_git_credential**](DefaultApi.md#delete_git_credential) | **DELETE** /git-credentials/{credential_id} | Deletes the credential
[**get_git_credential**](DefaultApi.md#get_git_credential) | **GET** /git-credentials/{credential_id} | Get a credential entry
[**get_git_credential_list**](DefaultApi.md#get_git_credential_list) | **GET** /git-credentials | Get Git credentials
[**update_git_credential**](DefaultApi.md#update_git_credential) | **PATCH** /git-credentials/{credential_id} | Updates the credential



## create_git_credential

> crate::models::GetCredentialResponse create_git_credential(create_credential_request)
Create a Git credential entry

Creates a Git credential entry for the user. Only one Git credential per user is supported, so any attempts to create credentials if an entry already exists will fail. Use the PATCH endpoint to update existing credentials, or the DELETE endpoint to delete existing credentials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_credential_request** | [**CreateCredentialRequest**](CreateCredentialRequest.md) | Details required to create a Git credential entry | [required] |

### Return type

[**crate::models::GetCredentialResponse**](GetCredentialResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_git_credential

> delete_git_credential(credential_id)
Deletes the credential

Deletes the specified credential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **String** | The ID for the corresponding credential to access. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_git_credential

> crate::models::GetCredentialResponse get_git_credential(credential_id)
Get a credential entry

Returns the credential with the given credential ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **String** | The ID for the corresponding credential to access. | [required] |

### Return type

[**crate::models::GetCredentialResponse**](GetCredentialResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_git_credential_list

> crate::models::GetCredentialsResponse get_git_credential_list()
Get Git credentials

Returns the calling user's Git credentials. One credential per user is supported.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetCredentialsResponse**](GetCredentialsResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_git_credential

> crate::models::GetCredentialResponse update_git_credential(credential_id, update_credential_request)
Updates the credential

Updates the credential.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **String** | The ID for the corresponding credential to access. | [required] |
**update_credential_request** | [**UpdateCredentialRequest**](UpdateCredentialRequest.md) | Details required to update the credential | [required] |

### Return type

[**crate::models::GetCredentialResponse**](GetCredentialResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

