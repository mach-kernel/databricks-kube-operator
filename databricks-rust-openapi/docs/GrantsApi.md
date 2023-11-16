# \GrantsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**grantsget**](GrantsApi.md#grantsget) | **GET** /api/2.1/unity-catalog/permissions/{securable_type}/{full_name} | Get permissions
[**grantsget_effective**](GrantsApi.md#grantsget_effective) | **GET** /api/2.1/unity-catalog/effective-permissions/{securable_type}/{full_name} | Get effective permissions
[**grantsupdate**](GrantsApi.md#grantsupdate) | **PATCH** /api/2.1/unity-catalog/permissions/{securable_type}/{full_name} | Update permissions



## grantsget

> crate::models::CatalogPermissionsList grantsget(securable_type, full_name, principal)
Get permissions

Gets the permissions for a securable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**securable_type** | [**CatalogSecurableType**](.md) | Type of securable. | [required] |
**full_name** | [**serde_json::Value**](.md) | Full name of securable. | [required] |
**principal** | Option<**String**> | If provided, only the permissions for the specified principal (user or group) are returned. |  |

### Return type

[**crate::models::CatalogPermissionsList**](CatalogPermissionsList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grantsget_effective

> crate::models::CatalogEffectivePermissionsList grantsget_effective(securable_type, full_name, principal)
Get effective permissions

Gets the effective permissions for a securable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**securable_type** | [**CatalogSecurableType**](.md) | Type of securable. | [required] |
**full_name** | [**serde_json::Value**](.md) | Full name of securable. | [required] |
**principal** | Option<**String**> | If provided, only the effective permissions for the specified principal (user or group) are returned. |  |

### Return type

[**crate::models::CatalogEffectivePermissionsList**](CatalogEffectivePermissionsList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grantsupdate

> crate::models::CatalogPermissionsList grantsupdate(securable_type, full_name, catalog_update_permissions)
Update permissions

Updates the permissions for a securable.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**securable_type** | [**CatalogSecurableType**](.md) | Type of securable. | [required] |
**full_name** | [**serde_json::Value**](.md) | Full name of securable. | [required] |
**catalog_update_permissions** | Option<[**CatalogUpdatePermissions**](CatalogUpdatePermissions.md)> |  |  |

### Return type

[**crate::models::CatalogPermissionsList**](CatalogPermissionsList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

