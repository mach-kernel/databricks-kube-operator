# \ServingEndpointsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**serving_endpointsbuild_logs**](ServingEndpointsApi.md#serving_endpointsbuild_logs) | **GET** /api/2.0/serving-endpoints/{name}/served-models/{served_model_name}/build-logs | Retrieve the logs associated with building the model's environment for a given serving endpoint's served model.
[**serving_endpointscreate**](ServingEndpointsApi.md#serving_endpointscreate) | **POST** /api/2.0/serving-endpoints | Create a new serving endpoint
[**serving_endpointsdelete**](ServingEndpointsApi.md#serving_endpointsdelete) | **DELETE** /api/2.0/serving-endpoints/{name} | Delete a serving endpoint
[**serving_endpointsexport_metrics**](ServingEndpointsApi.md#serving_endpointsexport_metrics) | **GET** /api/2.0/serving-endpoints/{name}/metrics | Retrieve the metrics associated with a serving endpoint
[**serving_endpointsget**](ServingEndpointsApi.md#serving_endpointsget) | **GET** /api/2.0/serving-endpoints/{name} | Get a single serving endpoint
[**serving_endpointsget_serving_endpoint_permission_levels**](ServingEndpointsApi.md#serving_endpointsget_serving_endpoint_permission_levels) | **GET** /api/2.0/permissions/serving-endpoints/{serving_endpoint_id}/permissionLevels | Get serving endpoint permission levels
[**serving_endpointsget_serving_endpoint_permissions**](ServingEndpointsApi.md#serving_endpointsget_serving_endpoint_permissions) | **GET** /api/2.0/permissions/serving-endpoints/{serving_endpoint_id} | Get serving endpoint permissions
[**serving_endpointslist**](ServingEndpointsApi.md#serving_endpointslist) | **GET** /api/2.0/serving-endpoints | Retrieve all serving endpoints
[**serving_endpointslogs**](ServingEndpointsApi.md#serving_endpointslogs) | **GET** /api/2.0/serving-endpoints/{name}/served-models/{served_model_name}/logs | Retrieve the most recent log lines associated with a given serving endpoint's served model
[**serving_endpointsquery**](ServingEndpointsApi.md#serving_endpointsquery) | **POST** /serving-endpoints/{name}/invocations | Query a serving endpoint with provided model input.
[**serving_endpointsset_serving_endpoint_permissions**](ServingEndpointsApi.md#serving_endpointsset_serving_endpoint_permissions) | **PUT** /api/2.0/permissions/serving-endpoints/{serving_endpoint_id} | Set serving endpoint permissions
[**serving_endpointsupdate_config**](ServingEndpointsApi.md#serving_endpointsupdate_config) | **PUT** /api/2.0/serving-endpoints/{name}/config | Update a serving endpoint with a new config.
[**serving_endpointsupdate_serving_endpoint_permissions**](ServingEndpointsApi.md#serving_endpointsupdate_serving_endpoint_permissions) | **PATCH** /api/2.0/permissions/serving-endpoints/{serving_endpoint_id} | Update serving endpoint permissions



## serving_endpointsbuild_logs

> crate::models::ServingBuildLogsResponse serving_endpointsbuild_logs(name, served_model_name)
Retrieve the logs associated with building the model's environment for a given serving endpoint's served model.

Retrieves the build logs associated with the provided served model. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the serving endpoint that the served model belongs to. This field is required. | [required] |
**served_model_name** | **String** | The name of the served model that build logs will be retrieved for. This field is required. | [required] |

### Return type

[**crate::models::ServingBuildLogsResponse**](ServingBuildLogsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## serving_endpointscreate

> crate::models::ServingServingEndpointDetailed serving_endpointscreate(serving_create_serving_endpoint)
Create a new serving endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**serving_create_serving_endpoint** | [**ServingCreateServingEndpoint**](ServingCreateServingEndpoint.md) |  | [required] |

### Return type

[**crate::models::ServingServingEndpointDetailed**](ServingServingEndpointDetailed.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## serving_endpointsdelete

> serde_json::Value serving_endpointsdelete(name)
Delete a serving endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the serving endpoint. This field is required. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## serving_endpointsexport_metrics

> String serving_endpointsexport_metrics(name)
Retrieve the metrics associated with a serving endpoint

Retrieves the metrics associated with the provided serving endpoint in either Prometheus or OpenMetrics exposition format. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the serving endpoint to retrieve metrics for. This field is required. | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/openmetrics-text; version=1.0.0, text/plain; version=0.0.4; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## serving_endpointsget

> crate::models::ServingServingEndpointDetailed serving_endpointsget(name)
Get a single serving endpoint

Retrieves the details for a single serving endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the serving endpoint. This field is required. | [required] |

### Return type

[**crate::models::ServingServingEndpointDetailed**](ServingServingEndpointDetailed.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## serving_endpointsget_serving_endpoint_permission_levels

> crate::models::ServingGetServingEndpointPermissionLevelsResponse serving_endpointsget_serving_endpoint_permission_levels(serving_endpoint_id)
Get serving endpoint permission levels

Gets the permission levels that a user can have on an object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**serving_endpoint_id** | **String** | The serving endpoint for which to get or manage permissions. | [required] |

### Return type

[**crate::models::ServingGetServingEndpointPermissionLevelsResponse**](ServingGetServingEndpointPermissionLevelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## serving_endpointsget_serving_endpoint_permissions

> crate::models::ServingServingEndpointPermissions serving_endpointsget_serving_endpoint_permissions(serving_endpoint_id)
Get serving endpoint permissions

Gets the permissions of a serving endpoint. Serving endpoints can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**serving_endpoint_id** | [**serde_json::Value**](.md) | The serving endpoint for which to get or manage permissions. | [required] |

### Return type

[**crate::models::ServingServingEndpointPermissions**](ServingServingEndpointPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## serving_endpointslist

> crate::models::ServingListEndpointsResponse serving_endpointslist()
Retrieve all serving endpoints

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ServingListEndpointsResponse**](ServingListEndpointsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## serving_endpointslogs

> crate::models::ServingServerLogsResponse serving_endpointslogs(name, served_model_name)
Retrieve the most recent log lines associated with a given serving endpoint's served model

Retrieves the service logs associated with the provided served model. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the serving endpoint that the served model belongs to. This field is required. | [required] |
**served_model_name** | **String** | The name of the served model that logs will be retrieved for. This field is required. | [required] |

### Return type

[**crate::models::ServingServerLogsResponse**](ServingServerLogsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## serving_endpointsquery

> crate::models::ServingQueryEndpointResponse serving_endpointsquery(name)
Query a serving endpoint with provided model input.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the serving endpoint. This field is required. | [required] |

### Return type

[**crate::models::ServingQueryEndpointResponse**](ServingQueryEndpointResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## serving_endpointsset_serving_endpoint_permissions

> crate::models::ServingServingEndpointPermissions serving_endpointsset_serving_endpoint_permissions(serving_endpoint_id, serving_serving_endpoint_permissions_request)
Set serving endpoint permissions

Sets permissions on a serving endpoint. Serving endpoints can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**serving_endpoint_id** | [**serde_json::Value**](.md) | The serving endpoint for which to get or manage permissions. | [required] |
**serving_serving_endpoint_permissions_request** | Option<[**ServingServingEndpointPermissionsRequest**](ServingServingEndpointPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::ServingServingEndpointPermissions**](ServingServingEndpointPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## serving_endpointsupdate_config

> crate::models::ServingServingEndpointDetailed serving_endpointsupdate_config(name, serving_endpoint_core_config_input)
Update a serving endpoint with a new config.

Updates any combination of the serving endpoint's served models, the compute  configuration of those served models, and the endpoint's traffic config. An endpoint that already has an update in progress can not be updated until the current update completes or fails. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the serving endpoint to update. This field is required. | [required] |
**serving_endpoint_core_config_input** | [**ServingEndpointCoreConfigInput**](ServingEndpointCoreConfigInput.md) |  | [required] |

### Return type

[**crate::models::ServingServingEndpointDetailed**](ServingServingEndpointDetailed.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## serving_endpointsupdate_serving_endpoint_permissions

> crate::models::ServingServingEndpointPermissions serving_endpointsupdate_serving_endpoint_permissions(serving_endpoint_id, serving_serving_endpoint_permissions_request)
Update serving endpoint permissions

Updates the permissions on a serving endpoint. Serving endpoints can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**serving_endpoint_id** | [**serde_json::Value**](.md) | The serving endpoint for which to get or manage permissions. | [required] |
**serving_serving_endpoint_permissions_request** | Option<[**ServingServingEndpointPermissionsRequest**](ServingServingEndpointPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::ServingServingEndpointPermissions**](ServingServingEndpointPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

