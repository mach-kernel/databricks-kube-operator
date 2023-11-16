# \TokenApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tokenscreate**](TokenApi.md#tokenscreate) | **POST** /api/2.0/token/create | Create a user token
[**tokenslist_tokens**](TokenApi.md#tokenslist_tokens) | **GET** /api/2.0/token/list | List tokens
[**tokensrevoke_token**](TokenApi.md#tokensrevoke_token) | **POST** /api/2.0/token/delete | Revoke token



## tokenscreate

> crate::models::SettingsCreateTokenResponse tokenscreate(settings_create_token_request)
Create a user token

Creates and returns a token for a user. If this call is made through token authentication, it creates a token with the same client ID as the authenticated token. If the user's token quota is exceeded, this call returns an error **QUOTA_EXCEEDED**. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**settings_create_token_request** | Option<[**SettingsCreateTokenRequest**](SettingsCreateTokenRequest.md)> |  |  |

### Return type

[**crate::models::SettingsCreateTokenResponse**](SettingsCreateTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokenslist_tokens

> crate::models::SettingsListTokensResponse tokenslist_tokens()
List tokens

Lists all the valid tokens for a user-workspace pair.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SettingsListTokensResponse**](SettingsListTokensResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokensrevoke_token

> serde_json::Value tokensrevoke_token(settings_revoke_token_request)
Revoke token

Revokes an access token.  If a token with the specified ID is not valid, this call returns an error **RESOURCE_DOES_NOT_EXIST**. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**settings_revoke_token_request** | Option<[**SettingsRevokeTokenRequest**](SettingsRevokeTokenRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

