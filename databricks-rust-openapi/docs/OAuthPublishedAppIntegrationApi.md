# \OAuthPublishedAppIntegrationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**published_app_integrationcreate**](OAuthPublishedAppIntegrationApi.md#published_app_integrationcreate) | **POST** /api/2.0/accounts/{account_id}/oauth2/published-app-integrations | Create Published OAuth App Integration
[**published_app_integrationdelete**](OAuthPublishedAppIntegrationApi.md#published_app_integrationdelete) | **DELETE** /api/2.0/accounts/{account_id}/oauth2/published-app-integrations/{integration_id} | Delete Published OAuth App Integration
[**published_app_integrationget**](OAuthPublishedAppIntegrationApi.md#published_app_integrationget) | **GET** /api/2.0/accounts/{account_id}/oauth2/published-app-integrations/{integration_id} | Get OAuth Published App Integration
[**published_app_integrationlist**](OAuthPublishedAppIntegrationApi.md#published_app_integrationlist) | **GET** /api/2.0/accounts/{account_id}/oauth2/published-app-integrations | Get published oauth app integrations
[**published_app_integrationupdate**](OAuthPublishedAppIntegrationApi.md#published_app_integrationupdate) | **PATCH** /api/2.0/accounts/{account_id}/oauth2/published-app-integrations/{integration_id} | Updates Published OAuth App Integration



## published_app_integrationcreate

> crate::models::Oauth2CreatePublishedAppIntegrationOutput published_app_integrationcreate(account_id, oauth2_create_published_app_integration)
Create Published OAuth App Integration

Create Published OAuth App Integration.  You can retrieve the published oauth app integration via :method:PublishedAppIntegration/get. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The account ID. | [required] |
**oauth2_create_published_app_integration** | Option<[**Oauth2CreatePublishedAppIntegration**](Oauth2CreatePublishedAppIntegration.md)> |  |  |

### Return type

[**crate::models::Oauth2CreatePublishedAppIntegrationOutput**](Oauth2CreatePublishedAppIntegrationOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## published_app_integrationdelete

> serde_json::Value published_app_integrationdelete(account_id, integration_id)
Delete Published OAuth App Integration

Delete an existing Published OAuth App Integration. You can retrieve the published oauth app integration via :method:PublishedAppIntegration/get. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | The account ID. | [required] |
**integration_id** | [**serde_json::Value**](.md) | The oauth app integration ID. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## published_app_integrationget

> crate::models::Oauth2GetPublishedAppIntegrationOutput published_app_integrationget(account_id, integration_id)
Get OAuth Published App Integration

Gets the Published OAuth App Integration for the given integration id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | The account ID. | [required] |
**integration_id** | [**serde_json::Value**](.md) | The oauth app integration ID. | [required] |

### Return type

[**crate::models::Oauth2GetPublishedAppIntegrationOutput**](Oauth2GetPublishedAppIntegrationOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## published_app_integrationlist

> crate::models::Oauth2GetPublishedAppIntegrationsOutput published_app_integrationlist(account_id)
Get published oauth app integrations

Get the list of published oauth app integrations for the specified Databricks account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The account ID. | [required] |

### Return type

[**crate::models::Oauth2GetPublishedAppIntegrationsOutput**](Oauth2GetPublishedAppIntegrationsOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## published_app_integrationupdate

> serde_json::Value published_app_integrationupdate(account_id, integration_id, oauth2_update_published_app_integration)
Updates Published OAuth App Integration

Updates an existing published OAuth App Integration. You can retrieve the published oauth app integration via :method:PublishedAppIntegration/get. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | The account ID. | [required] |
**integration_id** | [**serde_json::Value**](.md) | The oauth app integration ID. | [required] |
**oauth2_update_published_app_integration** | Option<[**Oauth2UpdatePublishedAppIntegration**](Oauth2UpdatePublishedAppIntegration.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

