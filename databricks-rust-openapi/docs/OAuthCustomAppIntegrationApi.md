# \OAuthCustomAppIntegrationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**custom_app_integrationcreate**](OAuthCustomAppIntegrationApi.md#custom_app_integrationcreate) | **POST** /api/2.0/accounts/{account_id}/oauth2/custom-app-integrations | Create Custom OAuth App Integration
[**custom_app_integrationdelete**](OAuthCustomAppIntegrationApi.md#custom_app_integrationdelete) | **DELETE** /api/2.0/accounts/{account_id}/oauth2/custom-app-integrations/{integration_id} | Delete Custom OAuth App Integration
[**custom_app_integrationget**](OAuthCustomAppIntegrationApi.md#custom_app_integrationget) | **GET** /api/2.0/accounts/{account_id}/oauth2/custom-app-integrations/{integration_id} | Get OAuth Custom App Integration
[**custom_app_integrationlist**](OAuthCustomAppIntegrationApi.md#custom_app_integrationlist) | **GET** /api/2.0/accounts/{account_id}/oauth2/custom-app-integrations | Get custom oauth app integrations
[**custom_app_integrationupdate**](OAuthCustomAppIntegrationApi.md#custom_app_integrationupdate) | **PATCH** /api/2.0/accounts/{account_id}/oauth2/custom-app-integrations/{integration_id} | Updates Custom OAuth App Integration



## custom_app_integrationcreate

> crate::models::Oauth2CreateCustomAppIntegrationOutput custom_app_integrationcreate(account_id, oauth2_create_custom_app_integration)
Create Custom OAuth App Integration

Create Custom OAuth App Integration.  You can retrieve the custom oauth app integration via :method:CustomAppIntegration/get. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | The account ID. | [required] |
**oauth2_create_custom_app_integration** | Option<[**Oauth2CreateCustomAppIntegration**](Oauth2CreateCustomAppIntegration.md)> |  |  |

### Return type

[**crate::models::Oauth2CreateCustomAppIntegrationOutput**](Oauth2CreateCustomAppIntegrationOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_app_integrationdelete

> serde_json::Value custom_app_integrationdelete(account_id, integration_id)
Delete Custom OAuth App Integration

Delete an existing Custom OAuth App Integration. You can retrieve the custom oauth app integration via :method:CustomAppIntegration/get. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | The account ID. | [required] |
**integration_id** | **String** | The oauth app integration ID. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_app_integrationget

> crate::models::Oauth2GetCustomAppIntegrationOutput custom_app_integrationget(account_id, integration_id)
Get OAuth Custom App Integration

Gets the Custom OAuth App Integration for the given integration id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | The account ID. | [required] |
**integration_id** | **String** | The oauth app integration ID. | [required] |

### Return type

[**crate::models::Oauth2GetCustomAppIntegrationOutput**](Oauth2GetCustomAppIntegrationOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_app_integrationlist

> crate::models::Oauth2GetCustomAppIntegrationsOutput custom_app_integrationlist(account_id)
Get custom oauth app integrations

Get the list of custom oauth app integrations for the specified Databricks account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | The account ID. | [required] |

### Return type

[**crate::models::Oauth2GetCustomAppIntegrationsOutput**](Oauth2GetCustomAppIntegrationsOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_app_integrationupdate

> serde_json::Value custom_app_integrationupdate(account_id, integration_id, oauth2_update_custom_app_integration)
Updates Custom OAuth App Integration

Updates an existing custom OAuth App Integration. You can retrieve the custom oauth app integration via :method:CustomAppIntegration/get. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | The account ID. | [required] |
**integration_id** | **String** | The oauth app integration ID. | [required] |
**oauth2_update_custom_app_integration** | Option<[**Oauth2UpdateCustomAppIntegration**](Oauth2UpdateCustomAppIntegration.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

