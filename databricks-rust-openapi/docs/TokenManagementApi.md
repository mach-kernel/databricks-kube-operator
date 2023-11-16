# \TokenManagementApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**token_managementcreate_obo_token**](TokenManagementApi.md#token_managementcreate_obo_token) | **POST** /api/2.0/token-management/on-behalf-of/tokens | Create on-behalf token
[**token_managementdelete**](TokenManagementApi.md#token_managementdelete) | **DELETE** /api/2.0/token-management/tokens/{token_id} | Delete a token
[**token_managementget**](TokenManagementApi.md#token_managementget) | **GET** /api/2.0/token-management/tokens/{token_id} | Get token info
[**token_managementget_token_permission_levels**](TokenManagementApi.md#token_managementget_token_permission_levels) | **GET** /api/2.0/permissions/authorization/tokens/permissionLevels | Get token permission levels
[**token_managementget_token_permissions**](TokenManagementApi.md#token_managementget_token_permissions) | **GET** /api/2.0/permissions/authorization/tokens | Get token permissions
[**token_managementlist**](TokenManagementApi.md#token_managementlist) | **GET** /api/2.0/token-management/tokens | List all tokens
[**token_managementset_token_permissions**](TokenManagementApi.md#token_managementset_token_permissions) | **PUT** /api/2.0/permissions/authorization/tokens | Set token permissions
[**token_managementupdate_token_permissions**](TokenManagementApi.md#token_managementupdate_token_permissions) | **PATCH** /api/2.0/permissions/authorization/tokens | Update token permissions



## token_managementcreate_obo_token

> crate::models::SettingsCreateOboTokenResponse token_managementcreate_obo_token(settings_create_obo_token_request)
Create on-behalf token

Creates a token on behalf of a service principal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**settings_create_obo_token_request** | [**SettingsCreateOboTokenRequest**](SettingsCreateOboTokenRequest.md) | Configuration details for creating on-behalf tokens. | [required] |

### Return type

[**crate::models::SettingsCreateOboTokenResponse**](SettingsCreateOboTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_managementdelete

> token_managementdelete(token_id)
Delete a token

Deletes a token, specified by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **String** | The ID of the token to get. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_managementget

> crate::models::SettingsTokenInfo token_managementget(token_id)
Get token info

Gets information about a token, specified by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_id** | **String** | The ID of the token to get. | [required] |

### Return type

[**crate::models::SettingsTokenInfo**](SettingsTokenInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_managementget_token_permission_levels

> crate::models::SettingsGetTokenPermissionLevelsResponse token_managementget_token_permission_levels()
Get token permission levels

Gets the permission levels that a user can have on an object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SettingsGetTokenPermissionLevelsResponse**](SettingsGetTokenPermissionLevelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_managementget_token_permissions

> crate::models::SettingsTokenPermissions token_managementget_token_permissions()
Get token permissions

Gets the permissions of all tokens. Tokens can inherit permissions from their root object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SettingsTokenPermissions**](SettingsTokenPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_managementlist

> crate::models::SettingsListTokensResponse token_managementlist(created_by_id, created_by_username)
List all tokens

Lists all tokens associated with the specified workspace or user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**created_by_id** | Option<**String**> | User ID of the user that created the token. |  |
**created_by_username** | Option<**String**> | Username of the user that created the token. |  |

### Return type

[**crate::models::SettingsListTokensResponse**](SettingsListTokensResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_managementset_token_permissions

> crate::models::SettingsTokenPermissions token_managementset_token_permissions(settings_token_permissions_request)
Set token permissions

Sets permissions on all tokens. Tokens can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**settings_token_permissions_request** | Option<[**SettingsTokenPermissionsRequest**](SettingsTokenPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::SettingsTokenPermissions**](SettingsTokenPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## token_managementupdate_token_permissions

> crate::models::SettingsTokenPermissions token_managementupdate_token_permissions(settings_token_permissions_request)
Update token permissions

Updates the permissions on all tokens. Tokens can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**settings_token_permissions_request** | Option<[**SettingsTokenPermissionsRequest**](SettingsTokenPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::SettingsTokenPermissions**](SettingsTokenPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

