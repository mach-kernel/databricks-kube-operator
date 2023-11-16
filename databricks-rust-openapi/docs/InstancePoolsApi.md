# \InstancePoolsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**instance_poolscreate**](InstancePoolsApi.md#instance_poolscreate) | **POST** /api/2.0/instance-pools/create | Create a new instance pool
[**instance_poolsdelete**](InstancePoolsApi.md#instance_poolsdelete) | **POST** /api/2.0/instance-pools/delete | Delete an instance pool
[**instance_poolsedit**](InstancePoolsApi.md#instance_poolsedit) | **POST** /api/2.0/instance-pools/edit | Edit an existing instance pool
[**instance_poolsget**](InstancePoolsApi.md#instance_poolsget) | **GET** /api/2.0/instance-pools/get | Get instance pool information
[**instance_poolsget_instance_pool_permission_levels**](InstancePoolsApi.md#instance_poolsget_instance_pool_permission_levels) | **GET** /api/2.0/permissions/instance-pools/{instance_pool_id}/permissionLevels | Get instance pool permission levels
[**instance_poolsget_instance_pool_permissions**](InstancePoolsApi.md#instance_poolsget_instance_pool_permissions) | **GET** /api/2.0/permissions/instance-pools/{instance_pool_id} | Get instance pool permissions
[**instance_poolslist**](InstancePoolsApi.md#instance_poolslist) | **GET** /api/2.0/instance-pools/list | List instance pool info
[**instance_poolsset_instance_pool_permissions**](InstancePoolsApi.md#instance_poolsset_instance_pool_permissions) | **PUT** /api/2.0/permissions/instance-pools/{instance_pool_id} | Set instance pool permissions
[**instance_poolsupdate_instance_pool_permissions**](InstancePoolsApi.md#instance_poolsupdate_instance_pool_permissions) | **PATCH** /api/2.0/permissions/instance-pools/{instance_pool_id} | Update instance pool permissions



## instance_poolscreate

> crate::models::ComputeCreateInstancePoolResponse instance_poolscreate(compute_create_instance_pool)
Create a new instance pool

Creates a new instance pool using idle and ready-to-use cloud instances.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_create_instance_pool** | Option<[**ComputeCreateInstancePool**](ComputeCreateInstancePool.md)> |  |  |

### Return type

[**crate::models::ComputeCreateInstancePoolResponse**](ComputeCreateInstancePoolResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_poolsdelete

> serde_json::Value instance_poolsdelete(compute_delete_instance_pool)
Delete an instance pool

Deletes the instance pool permanently. The idle instances in the pool are terminated asynchronously. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_delete_instance_pool** | Option<[**ComputeDeleteInstancePool**](ComputeDeleteInstancePool.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_poolsedit

> serde_json::Value instance_poolsedit(compute_edit_instance_pool)
Edit an existing instance pool

Modifies the configuration of an existing instance pool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_edit_instance_pool** | Option<[**ComputeEditInstancePool**](ComputeEditInstancePool.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_poolsget

> crate::models::ComputeGetInstancePool instance_poolsget(instance_pool_id)
Get instance pool information

Retrieve the information for an instance pool based on its identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_pool_id** | **String** | The canonical unique identifier for the instance pool. | [required] |

### Return type

[**crate::models::ComputeGetInstancePool**](ComputeGetInstancePool.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_poolsget_instance_pool_permission_levels

> crate::models::ComputeGetInstancePoolPermissionLevelsResponse instance_poolsget_instance_pool_permission_levels(instance_pool_id)
Get instance pool permission levels

Gets the permission levels that a user can have on an object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_pool_id** | **String** | The instance pool for which to get or manage permissions. | [required] |

### Return type

[**crate::models::ComputeGetInstancePoolPermissionLevelsResponse**](ComputeGetInstancePoolPermissionLevelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_poolsget_instance_pool_permissions

> crate::models::ComputeInstancePoolPermissions instance_poolsget_instance_pool_permissions(instance_pool_id)
Get instance pool permissions

Gets the permissions of an instance pool. Instance pools can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_pool_id** | [**serde_json::Value**](.md) | The instance pool for which to get or manage permissions. | [required] |

### Return type

[**crate::models::ComputeInstancePoolPermissions**](ComputeInstancePoolPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_poolslist

> crate::models::ComputeListInstancePools instance_poolslist()
List instance pool info

Gets a list of instance pools with their statistics.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ComputeListInstancePools**](ComputeListInstancePools.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_poolsset_instance_pool_permissions

> crate::models::ComputeInstancePoolPermissions instance_poolsset_instance_pool_permissions(instance_pool_id, compute_instance_pool_permissions_request)
Set instance pool permissions

Sets permissions on an instance pool. Instance pools can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_pool_id** | [**serde_json::Value**](.md) | The instance pool for which to get or manage permissions. | [required] |
**compute_instance_pool_permissions_request** | Option<[**ComputeInstancePoolPermissionsRequest**](ComputeInstancePoolPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::ComputeInstancePoolPermissions**](ComputeInstancePoolPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_poolsupdate_instance_pool_permissions

> crate::models::ComputeInstancePoolPermissions instance_poolsupdate_instance_pool_permissions(instance_pool_id, compute_instance_pool_permissions_request)
Update instance pool permissions

Updates the permissions on an instance pool. Instance pools can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_pool_id** | [**serde_json::Value**](.md) | The instance pool for which to get or manage permissions. | [required] |
**compute_instance_pool_permissions_request** | Option<[**ComputeInstancePoolPermissionsRequest**](ComputeInstancePoolPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::ComputeInstancePoolPermissions**](ComputeInstancePoolPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

