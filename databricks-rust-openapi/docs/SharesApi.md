# \SharesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sharescreate**](SharesApi.md#sharescreate) | **POST** /api/2.1/unity-catalog/shares | Create a share
[**sharesdelete**](SharesApi.md#sharesdelete) | **DELETE** /api/2.1/unity-catalog/shares/{name} | Delete a share
[**sharesget**](SharesApi.md#sharesget) | **GET** /api/2.1/unity-catalog/shares/{name} | Get a share
[**shareslist**](SharesApi.md#shareslist) | **GET** /api/2.1/unity-catalog/shares | List shares
[**sharesshare_permissions**](SharesApi.md#sharesshare_permissions) | **GET** /api/2.1/unity-catalog/shares/{name}/permissions | Get permissions
[**sharesupdate**](SharesApi.md#sharesupdate) | **PATCH** /api/2.1/unity-catalog/shares/{name} | Update a share
[**sharesupdate_permissions**](SharesApi.md#sharesupdate_permissions) | **PATCH** /api/2.1/unity-catalog/shares/{name}/permissions | Update permissions



## sharescreate

> crate::models::SharingShareInfo sharescreate(sharing_create_share)
Create a share

Creates a new share for data objects. Data objects can be added after creation with **update**. The caller must be a metastore admin or have the **CREATE_SHARE** privilege on the metastore. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_create_share** | Option<[**SharingCreateShare**](SharingCreateShare.md)> |  |  |

### Return type

[**crate::models::SharingShareInfo**](SharingShareInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sharesdelete

> serde_json::Value sharesdelete(name)
Delete a share

Deletes a data object share from the metastore. The caller must be an owner of the share.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the share. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sharesget

> crate::models::SharingShareInfo sharesget(name, include_shared_data)
Get a share

Gets a data object share from the metastore. The caller must be a metastore admin or the owner of the share.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the share. | [required] |
**include_shared_data** | Option<**bool**> | Query for data to include in the share. |  |

### Return type

[**crate::models::SharingShareInfo**](SharingShareInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## shareslist

> crate::models::SharingListSharesResponse shareslist()
List shares

Gets an array of data object shares from the metastore. The caller must be a metastore admin or the owner of the share. There is no guarantee of a specific ordering of the elements in the array. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SharingListSharesResponse**](SharingListSharesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sharesshare_permissions

> crate::models::CatalogPermissionsList sharesshare_permissions(name)
Get permissions

Gets the permissions for a data share from the metastore. The caller must be a metastore admin or the owner of the share. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**serde_json::Value**](.md) | The name of the share. | [required] |

### Return type

[**crate::models::CatalogPermissionsList**](CatalogPermissionsList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sharesupdate

> crate::models::SharingShareInfo sharesupdate(name, sharing_update_share)
Update a share

Updates the share with the changes and data objects in the request. The caller must be the owner of the share or a metastore admin.  When the caller is a metastore admin, only the __owner__ field can be updated.  In the case that the share name is changed, **updateShare** requires that the caller is both the share owner and a metastore admin.  For each table that is added through this method, the share owner must also have **SELECT** privilege on the table. This privilege must be maintained indefinitely for recipients to be able to access the table. Typically, you should use a group as the share owner.  Table removals through **update** do not require additional privileges. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the share. | [required] |
**sharing_update_share** | Option<[**SharingUpdateShare**](SharingUpdateShare.md)> |  |  |

### Return type

[**crate::models::SharingShareInfo**](SharingShareInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sharesupdate_permissions

> serde_json::Value sharesupdate_permissions(name, sharing_update_share_permissions)
Update permissions

Updates the permissions for a data share in the metastore. The caller must be a metastore admin or an owner of the share.  For new recipient grants, the user must also be the owner of the recipients. recipient revocations do not require additional privileges. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**serde_json::Value**](.md) | The name of the share. | [required] |
**sharing_update_share_permissions** | Option<[**SharingUpdateSharePermissions**](SharingUpdateSharePermissions.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

