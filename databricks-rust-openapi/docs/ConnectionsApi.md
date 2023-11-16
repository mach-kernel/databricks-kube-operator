# \ConnectionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**connectionscreate**](ConnectionsApi.md#connectionscreate) | **POST** /api/2.1/unity-catalog/connections | Create a connection
[**connectionsdelete**](ConnectionsApi.md#connectionsdelete) | **DELETE** /api/2.1/unity-catalog/connections/{name_arg} | Delete a connection
[**connectionsget**](ConnectionsApi.md#connectionsget) | **GET** /api/2.1/unity-catalog/connections/{name_arg} | Get a connection
[**connectionslist**](ConnectionsApi.md#connectionslist) | **GET** /api/2.1/unity-catalog/connections | List connections
[**connectionsupdate**](ConnectionsApi.md#connectionsupdate) | **PATCH** /api/2.1/unity-catalog/connections/{name_arg} | Update a connection



## connectionscreate

> crate::models::CatalogConnectionInfo connectionscreate(catalog_create_connection)
Create a connection

Creates a new connection  Creates a new connection to an external data source. It allows users to specify connection details and configurations for interaction with the external server. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_create_connection** | Option<[**CatalogCreateConnection**](CatalogCreateConnection.md)> |  |  |

### Return type

[**crate::models::CatalogConnectionInfo**](CatalogConnectionInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## connectionsdelete

> serde_json::Value connectionsdelete(name_arg)
Delete a connection

Deletes the connection that matches the supplied name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name_arg** | **String** | The name of the connection to be deleted. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## connectionsget

> crate::models::CatalogConnectionInfo connectionsget(name_arg)
Get a connection

Gets a connection from it's name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name_arg** | **String** | Name of the connection. | [required] |

### Return type

[**crate::models::CatalogConnectionInfo**](CatalogConnectionInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## connectionslist

> crate::models::CatalogListConnectionsResponse connectionslist()
List connections

List all connections. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CatalogListConnectionsResponse**](CatalogListConnectionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## connectionsupdate

> crate::models::CatalogConnectionInfo connectionsupdate(name_arg, catalog_update_connection)
Update a connection

Updates the connection that matches the supplied name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name_arg** | **String** | Name of the connection. | [required] |
**catalog_update_connection** | Option<[**CatalogUpdateConnection**](CatalogUpdateConnection.md)> |  |  |

### Return type

[**crate::models::CatalogConnectionInfo**](CatalogConnectionInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

