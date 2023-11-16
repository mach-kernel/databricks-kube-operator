# \ProvidersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**providerscreate**](ProvidersApi.md#providerscreate) | **POST** /api/2.1/unity-catalog/providers | Create an auth provider
[**providersdelete**](ProvidersApi.md#providersdelete) | **DELETE** /api/2.1/unity-catalog/providers/{name} | Delete a provider
[**providersget**](ProvidersApi.md#providersget) | **GET** /api/2.1/unity-catalog/providers/{name} | Get a provider
[**providerslist**](ProvidersApi.md#providerslist) | **GET** /api/2.1/unity-catalog/providers | List providers
[**providerslist_shares**](ProvidersApi.md#providerslist_shares) | **GET** /api/2.1/unity-catalog/providers/{name}/shares | List shares by Provider
[**providersupdate**](ProvidersApi.md#providersupdate) | **PATCH** /api/2.1/unity-catalog/providers/{name} | Update a provider



## providerscreate

> crate::models::SharingProviderInfo providerscreate(sharing_create_provider)
Create an auth provider

Creates a new authentication provider minimally based on a name and authentication type. The caller must be an admin on the metastore. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_create_provider** | Option<[**SharingCreateProvider**](SharingCreateProvider.md)> |  |  |

### Return type

[**crate::models::SharingProviderInfo**](SharingProviderInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providersdelete

> serde_json::Value providersdelete(name)
Delete a provider

Deletes an authentication provider, if the caller is a metastore admin or is the owner of the provider.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the provider. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providersget

> crate::models::SharingProviderInfo providersget(name)
Get a provider

Gets a specific authentication provider. The caller must supply the name of the provider, and must either be a metastore admin or the owner of the provider. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the provider. | [required] |

### Return type

[**crate::models::SharingProviderInfo**](SharingProviderInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providerslist

> crate::models::SharingListProvidersResponse providerslist(data_provider_global_metastore_id)
List providers

Gets an array of available authentication providers. The caller must either be a metastore admin or the owner of the providers. Providers not owned by the caller are not included in the response. There is no guarantee of a specific ordering of the elements in the array. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_provider_global_metastore_id** | Option<**String**> | If not provided, all providers will be returned. If no providers exist with this ID, no results will be returned. |  |

### Return type

[**crate::models::SharingListProvidersResponse**](SharingListProvidersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providerslist_shares

> crate::models::SharingListProviderSharesResponse providerslist_shares(name)
List shares by Provider

Gets an array of a specified provider's shares within the metastore where:    * the caller is a metastore admin, or   * the caller is the owner. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the provider in which to list shares. | [required] |

### Return type

[**crate::models::SharingListProviderSharesResponse**](SharingListProviderSharesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## providersupdate

> crate::models::SharingProviderInfo providersupdate(name, sharing_update_provider)
Update a provider

Updates the information for an authentication provider, if the caller is a metastore admin or is the owner of the provider. If the update changes the provider name, the caller must be both a metastore admin and the owner of the provider. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the provider. | [required] |
**sharing_update_provider** | Option<[**SharingUpdateProvider**](SharingUpdateProvider.md)> |  |  |

### Return type

[**crate::models::SharingProviderInfo**](SharingProviderInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

