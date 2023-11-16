# \AclPermissionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**dbsql_permissionsget**](AclPermissionsApi.md#dbsql_permissionsget) | **GET** /api/2.0/preview/sql/permissions/{objectType}/{objectId} | Get object ACL
[**dbsql_permissionsset**](AclPermissionsApi.md#dbsql_permissionsset) | **POST** /api/2.0/preview/sql/permissions/{objectType}/{objectId} | Set object ACL
[**dbsql_permissionstransfer_ownership**](AclPermissionsApi.md#dbsql_permissionstransfer_ownership) | **POST** /api/2.0/preview/sql/permissions/{objectType}/{objectId}/transfer | Transfer object ownership



## dbsql_permissionsget

> crate::models::DbsqlPermissionsget200Response dbsql_permissionsget(object_type, object_id)
Get object ACL

Gets a JSON representation of the access control list (ACL) for a specified object. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_type** | [**SqlobjectTypePlural**](.md) | The type of object permissions to check. | [required] |
**object_id** | **String** | Object ID. An ACL is returned for the object with this UUID. | [required] |

### Return type

[**crate::models::DbsqlPermissionsget200Response**](DbsqlPermissionsget_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dbsql_permissionsset

> crate::models::DbsqlPermissionsget200Response dbsql_permissionsset(object_type, object_id, dbsql_permissionsset_request)
Set object ACL

Sets the access control list (ACL) for a specified object. This operation will complete rewrite the ACL. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_type** | [**SqlobjectTypePlural**](.md) | The type of object permission to set. | [required] |
**object_id** | **String** | Object ID. The ACL for the object with this UUID is overwritten by this request's POST content. | [required] |
**dbsql_permissionsset_request** | [**DbsqlPermissionssetRequest**](DbsqlPermissionssetRequest.md) | An ACL list to be applied to the object specified in the URL. | [required] |

### Return type

[**crate::models::DbsqlPermissionsget200Response**](DbsqlPermissionsget_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dbsql_permissionstransfer_ownership

> crate::models::SqlSuccess dbsql_permissionstransfer_ownership(object_type, object_id, dbsql_permissionstransfer_ownership_request)
Transfer object ownership

Transfers ownership of a dashboard, query, or alert to an active user. Requires an admin API key. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_type** | [**SqlownableObjectType**](.md) | The type of object on which to change ownership. | [required] |
**object_id** | [**DbsqlPermissionstransferOwnershipRequest**](.md) | The ID of the object on which to change ownership. | [required] |
**dbsql_permissionstransfer_ownership_request** | [**DbsqlPermissionstransferOwnershipRequest**](DbsqlPermissionstransferOwnershipRequest.md) | Email address for the new owner, who must exist in the workspace. | [required] |

### Return type

[**crate::models::SqlSuccess**](SqlSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

