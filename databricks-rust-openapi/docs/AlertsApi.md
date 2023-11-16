# \AlertsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**alertscreate**](AlertsApi.md#alertscreate) | **POST** /api/2.0/preview/sql/alerts | Create an alert
[**alertsdelete**](AlertsApi.md#alertsdelete) | **DELETE** /api/2.0/preview/sql/alerts/{alert_id} | Delete an alert
[**alertsget**](AlertsApi.md#alertsget) | **GET** /api/2.0/preview/sql/alerts/{alert_id} | Get an alert
[**alertslist**](AlertsApi.md#alertslist) | **GET** /api/2.0/preview/sql/alerts | Get alerts
[**alertsupdate**](AlertsApi.md#alertsupdate) | **PUT** /api/2.0/preview/sql/alerts/{alert_id} | Update an alert



## alertscreate

> crate::models::SqlAlert alertscreate(sql_create_alert)
Create an alert

Creates an alert. An alert is a Databricks SQL object that periodically runs a query, evaluates a condition of its result, and notifies users or notification destinations if the condition was met. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sql_create_alert** | [**SqlCreateAlert**](SqlCreateAlert.md) |  | [required] |

### Return type

[**crate::models::SqlAlert**](SqlAlert.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alertsdelete

> serde_json::Value alertsdelete(alert_id)
Delete an alert

Deletes an alert. Deleted alerts are no longer accessible and cannot be restored. **Note:** Unlike queries and dashboards, alerts cannot be moved to the trash. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alertsget

> crate::models::SqlAlert alertsget(alert_id)
Get an alert

Gets an alert. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** |  | [required] |

### Return type

[**crate::models::SqlAlert**](SqlAlert.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alertslist

> Vec<crate::models::SqlAlert> alertslist()
Get alerts

Gets a list of alerts. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::SqlAlert>**](SqlAlert.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## alertsupdate

> serde_json::Value alertsupdate(alert_id, sql_edit_alert)
Update an alert

Updates an alert. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** |  | [required] |
**sql_edit_alert** | [**SqlEditAlert**](SqlEditAlert.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

