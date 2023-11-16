# \SqlWarehousesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**warehousescreate**](SqlWarehousesApi.md#warehousescreate) | **POST** /api/2.0/sql/warehouses | Create a warehouse
[**warehousesdelete**](SqlWarehousesApi.md#warehousesdelete) | **DELETE** /api/2.0/sql/warehouses/{id} | Delete a warehouse
[**warehousesedit**](SqlWarehousesApi.md#warehousesedit) | **POST** /api/2.0/sql/warehouses/{id}/edit | Update a warehouse
[**warehousesget**](SqlWarehousesApi.md#warehousesget) | **GET** /api/2.0/sql/warehouses/{id} | Get warehouse info
[**warehousesget_warehouse_permission_levels**](SqlWarehousesApi.md#warehousesget_warehouse_permission_levels) | **GET** /api/2.0/permissions/warehouses/{warehouse_id}/permissionLevels | Get SQL warehouse permission levels
[**warehousesget_warehouse_permissions**](SqlWarehousesApi.md#warehousesget_warehouse_permissions) | **GET** /api/2.0/permissions/warehouses/{warehouse_id} | Get SQL warehouse permissions
[**warehousesget_workspace_warehouse_config**](SqlWarehousesApi.md#warehousesget_workspace_warehouse_config) | **GET** /api/2.0/sql/config/warehouses | Get the workspace configuration
[**warehouseslist**](SqlWarehousesApi.md#warehouseslist) | **GET** /api/2.0/sql/warehouses | List warehouses
[**warehousesset_warehouse_permissions**](SqlWarehousesApi.md#warehousesset_warehouse_permissions) | **PUT** /api/2.0/permissions/warehouses/{warehouse_id} | Set SQL warehouse permissions
[**warehousesset_workspace_warehouse_config**](SqlWarehousesApi.md#warehousesset_workspace_warehouse_config) | **PUT** /api/2.0/sql/config/warehouses | Set the workspace configuration
[**warehousesstart**](SqlWarehousesApi.md#warehousesstart) | **POST** /api/2.0/sql/warehouses/{id}/start | Start a warehouse
[**warehousesstop**](SqlWarehousesApi.md#warehousesstop) | **POST** /api/2.0/sql/warehouses/{id}/stop | Stop a warehouse
[**warehousesupdate_warehouse_permissions**](SqlWarehousesApi.md#warehousesupdate_warehouse_permissions) | **PATCH** /api/2.0/permissions/warehouses/{warehouse_id} | Update SQL warehouse permissions



## warehousescreate

> crate::models::SqlCreateWarehouseResponse warehousescreate(sql_create_warehouse_request)
Create a warehouse

Creates a new SQL warehouse.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sql_create_warehouse_request** | Option<[**SqlCreateWarehouseRequest**](SqlCreateWarehouseRequest.md)> |  |  |

### Return type

[**crate::models::SqlCreateWarehouseResponse**](SqlCreateWarehouseResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## warehousesdelete

> serde_json::Value warehousesdelete(id)
Delete a warehouse

Deletes a SQL warehouse.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Required. Id of the SQL warehouse. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## warehousesedit

> serde_json::Value warehousesedit(id, sql_edit_warehouse_request)
Update a warehouse

Updates the configuration for a SQL warehouse.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Required. Id of the warehouse to configure. | [required] |
**sql_edit_warehouse_request** | Option<[**SqlEditWarehouseRequest**](SqlEditWarehouseRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## warehousesget

> crate::models::SqlGetWarehouseResponse warehousesget(id)
Get warehouse info

Gets the information for a single SQL warehouse.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Required. Id of the SQL warehouse. | [required] |

### Return type

[**crate::models::SqlGetWarehouseResponse**](SqlGetWarehouseResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## warehousesget_warehouse_permission_levels

> crate::models::SqlGetWarehousePermissionLevelsResponse warehousesget_warehouse_permission_levels(warehouse_id)
Get SQL warehouse permission levels

Gets the permission levels that a user can have on an object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**warehouse_id** | **String** | The SQL warehouse for which to get or manage permissions. | [required] |

### Return type

[**crate::models::SqlGetWarehousePermissionLevelsResponse**](SqlGetWarehousePermissionLevelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## warehousesget_warehouse_permissions

> crate::models::SqlWarehousePermissions warehousesget_warehouse_permissions(warehouse_id)
Get SQL warehouse permissions

Gets the permissions of a SQL warehouse. SQL warehouses can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**warehouse_id** | [**serde_json::Value**](.md) | The SQL warehouse for which to get or manage permissions. | [required] |

### Return type

[**crate::models::SqlWarehousePermissions**](SqlWarehousePermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## warehousesget_workspace_warehouse_config

> crate::models::SqlGetWorkspaceWarehouseConfigResponse warehousesget_workspace_warehouse_config()
Get the workspace configuration

Gets the workspace level configuration that is shared by all SQL warehouses in a workspace.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SqlGetWorkspaceWarehouseConfigResponse**](SqlGetWorkspaceWarehouseConfigResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## warehouseslist

> crate::models::SqlListWarehousesResponse warehouseslist(run_as_user_id)
List warehouses

Lists all SQL warehouses that a user has manager permissions on.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_as_user_id** | Option<**i32**> | Service Principal which will be used to fetch the list of warehouses. If not specified, the user from the session header is used. |  |

### Return type

[**crate::models::SqlListWarehousesResponse**](SqlListWarehousesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## warehousesset_warehouse_permissions

> crate::models::SqlWarehousePermissions warehousesset_warehouse_permissions(warehouse_id, sql_warehouse_permissions_request)
Set SQL warehouse permissions

Sets permissions on a SQL warehouse. SQL warehouses can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**warehouse_id** | [**serde_json::Value**](.md) | The SQL warehouse for which to get or manage permissions. | [required] |
**sql_warehouse_permissions_request** | Option<[**SqlWarehousePermissionsRequest**](SqlWarehousePermissionsRequest.md)> |  |  |

### Return type

[**crate::models::SqlWarehousePermissions**](SqlWarehousePermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## warehousesset_workspace_warehouse_config

> serde_json::Value warehousesset_workspace_warehouse_config(sql_set_workspace_warehouse_config_request)
Set the workspace configuration

Sets the workspace level configuration that is shared by all SQL warehouses in a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sql_set_workspace_warehouse_config_request** | Option<[**SqlSetWorkspaceWarehouseConfigRequest**](SqlSetWorkspaceWarehouseConfigRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## warehousesstart

> serde_json::Value warehousesstart(id)
Start a warehouse

Starts a SQL warehouse.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Required. Id of the SQL warehouse. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## warehousesstop

> serde_json::Value warehousesstop(id)
Stop a warehouse

Stops a SQL warehouse.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Required. Id of the SQL warehouse. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## warehousesupdate_warehouse_permissions

> crate::models::SqlWarehousePermissions warehousesupdate_warehouse_permissions(warehouse_id, sql_warehouse_permissions_request)
Update SQL warehouse permissions

Updates the permissions on a SQL warehouse. SQL warehouses can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**warehouse_id** | [**serde_json::Value**](.md) | The SQL warehouse for which to get or manage permissions. | [required] |
**sql_warehouse_permissions_request** | Option<[**SqlWarehousePermissionsRequest**](SqlWarehousePermissionsRequest.md)> |  |  |

### Return type

[**crate::models::SqlWarehousePermissions**](SqlWarehousePermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

