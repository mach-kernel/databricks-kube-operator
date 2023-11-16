# \RecipientActivationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**recipient_activationget_activation_url_info**](RecipientActivationApi.md#recipient_activationget_activation_url_info) | **GET** /api/2.1/unity-catalog/public/data_sharing_activation_info/{activation_url} | Get a share activation URL
[**recipient_activationretrieve_token**](RecipientActivationApi.md#recipient_activationretrieve_token) | **GET** /api/2.1/unity-catalog/public/data_sharing_activation/{activation_url} | Get an access token



## recipient_activationget_activation_url_info

> serde_json::Value recipient_activationget_activation_url_info(activation_url)
Get a share activation URL

Gets an activation URL for a share.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activation_url** | **String** | The one time activation url. It also accepts activation token. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recipient_activationretrieve_token

> crate::models::SharingRetrieveTokenResponse recipient_activationretrieve_token(activation_url)
Get an access token

Retrieve access token with an activation url. This is a public API without any authentication.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activation_url** | **String** | The one time activation url. It also accepts activation token. | [required] |

### Return type

[**crate::models::SharingRetrieveTokenResponse**](SharingRetrieveTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

