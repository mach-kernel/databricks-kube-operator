# \DashboardsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**dashboardscreate**](DashboardsApi.md#dashboardscreate) | **POST** /api/2.0/preview/sql/dashboards | Create a dashboard object
[**dashboardsdelete**](DashboardsApi.md#dashboardsdelete) | **DELETE** /api/2.0/preview/sql/dashboards/{dashboard_id} | Remove a dashboard
[**dashboardsget**](DashboardsApi.md#dashboardsget) | **GET** /api/2.0/preview/sql/dashboards/{dashboard_id} | Retrieve a definition
[**dashboardslist**](DashboardsApi.md#dashboardslist) | **GET** /api/2.0/preview/sql/dashboards | Get dashboard objects
[**dashboardsrestore**](DashboardsApi.md#dashboardsrestore) | **POST** /api/2.0/preview/sql/dashboards/trash/{dashboard_id} | Restore a dashboard



## dashboardscreate

> crate::models::SqlDashboard dashboardscreate(dashboardscreate_request)
Create a dashboard object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboardscreate_request** | [**DashboardscreateRequest**](DashboardscreateRequest.md) | Creates a new dashboard object. Only the `name` parameter is required in the POST request JSON body. Other fields can be included when duplicating dashboards with this API. Databricks does not recommend designing dashboards exclusively using this API.',  | [required] |

### Return type

[**crate::models::SqlDashboard**](SqlDashboard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dashboardsdelete

> serde_json::Value dashboardsdelete(dashboard_id)
Remove a dashboard

Moves a dashboard to the trash. Trashed dashboards do not appear in list views or searches, and cannot be shared. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dashboardsget

> crate::models::SqlDashboard dashboardsget(dashboard_id)
Retrieve a definition

Returns a JSON representation of a dashboard object, including its visualization and query objects. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**crate::models::SqlDashboard**](SqlDashboard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dashboardslist

> crate::models::Dashboardslist200Response dashboardslist(page_size, page, order, q)
Get dashboard objects

Fetch a paginated list of dashboard objects. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Number of dashboards to return per page. |  |
**page** | Option<**i32**> | Page number to retrieve. |  |
**order** | Option<**String**> | Name of dashboard attribute to order by. |  |
**q** | Option<**String**> | Full text search term. |  |

### Return type

[**crate::models::Dashboardslist200Response**](Dashboardslist_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dashboardsrestore

> serde_json::Value dashboardsrestore(dashboard_id)
Restore a dashboard

A restored dashboard appears in list views and searches and can be shared. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dashboard_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

