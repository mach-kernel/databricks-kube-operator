# \CurrentUserApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**current_userme**](CurrentUserApi.md#current_userme) | **GET** /api/2.0/preview/scim/v2/Me | Get current user info



## current_userme

> crate::models::IamUser current_userme()
Get current user info

Get details about the current method caller's identity.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IamUser**](IamUser.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

