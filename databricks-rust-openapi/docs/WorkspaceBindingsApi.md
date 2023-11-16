# \WorkspaceBindingsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**workspace_bindingsget**](WorkspaceBindingsApi.md#workspace_bindingsget) | **GET** /api/2.1/unity-catalog/workspace-bindings/catalogs/{name} | Get catalog workspace bindings
[**workspace_bindingsupdate**](WorkspaceBindingsApi.md#workspace_bindingsupdate) | **PATCH** /api/2.1/unity-catalog/workspace-bindings/catalogs/{name} | Update catalog workspace bindings



## workspace_bindingsget

> crate::models::CatalogCurrentWorkspaceBindings workspace_bindingsget(name)
Get catalog workspace bindings

Gets workspace bindings of the catalog. The caller must be a metastore admin or an owner of the catalog. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**serde_json::Value**](.md) | The name of the catalog. | [required] |

### Return type

[**crate::models::CatalogCurrentWorkspaceBindings**](CatalogCurrentWorkspaceBindings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_bindingsupdate

> crate::models::CatalogCurrentWorkspaceBindings workspace_bindingsupdate(name, catalog_update_workspace_bindings)
Update catalog workspace bindings

Updates workspace bindings of the catalog. The caller must be a metastore admin or an owner of the catalog. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**serde_json::Value**](.md) | The name of the catalog. | [required] |
**catalog_update_workspace_bindings** | Option<[**CatalogUpdateWorkspaceBindings**](CatalogUpdateWorkspaceBindings.md)> |  |  |

### Return type

[**crate::models::CatalogCurrentWorkspaceBindings**](CatalogCurrentWorkspaceBindings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

