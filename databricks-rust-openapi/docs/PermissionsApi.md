# \PermissionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**permissionsget**](PermissionsApi.md#permissionsget) | **GET** /api/2.0/permissions/{request_object_type}/{request_object_id} | Get object permissions
[**permissionsget_permission_levels**](PermissionsApi.md#permissionsget_permission_levels) | **GET** /api/2.0/permissions/{request_object_type}/{request_object_id}/permissionLevels | Get object permission levels
[**permissionsset**](PermissionsApi.md#permissionsset) | **PUT** /api/2.0/permissions/{request_object_type}/{request_object_id} | Set object permissions
[**permissionsupdate**](PermissionsApi.md#permissionsupdate) | **PATCH** /api/2.0/permissions/{request_object_type}/{request_object_id} | Update object permissions



## permissionsget

> crate::models::IamObjectPermissions permissionsget(request_object_type, request_object_id)
Get object permissions

Gets the permissions of an object. Objects can inherit permissions from their parent objects or root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_object_type** | **String** | <needs content> | [required] |
**request_object_id** | **String** |  | [required] |

### Return type

[**crate::models::IamObjectPermissions**](IamObjectPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permissionsget_permission_levels

> crate::models::IamGetPermissionLevelsResponse permissionsget_permission_levels(request_object_type, request_object_id)
Get object permission levels

Gets the permission levels that a user can have on an object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_object_type** | [**serde_json::Value**](.md) | <needs content> | [required] |
**request_object_id** | **String** | <needs content> | [required] |

### Return type

[**crate::models::IamGetPermissionLevelsResponse**](IamGetPermissionLevelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permissionsset

> crate::models::IamObjectPermissions permissionsset(request_object_type, request_object_id, iam_permissions_request)
Set object permissions

Sets permissions on an object. Objects can inherit permissions from their parent objects or root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_object_type** | **String** | <needs content> | [required] |
**request_object_id** | **String** |  | [required] |
**iam_permissions_request** | Option<[**IamPermissionsRequest**](IamPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::IamObjectPermissions**](IamObjectPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permissionsupdate

> crate::models::IamObjectPermissions permissionsupdate(request_object_type, request_object_id, iam_permissions_request)
Update object permissions

Updates the permissions on an object. Objects can inherit permissions from their parent objects or root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_object_type** | **String** | <needs content> | [required] |
**request_object_id** | **String** |  | [required] |
**iam_permissions_request** | Option<[**IamPermissionsRequest**](IamPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::IamObjectPermissions**](IamObjectPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

