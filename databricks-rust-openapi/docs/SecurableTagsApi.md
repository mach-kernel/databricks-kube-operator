# \SecurableTagsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**securable_tagslist**](SecurableTagsApi.md#securable_tagslist) | **GET** /api/2.1/unity-catalog/securable-tags/{securable_type}/{full_name} | Get tags for a securable
[**securable_tagsupdate**](SecurableTagsApi.md#securable_tagsupdate) | **PATCH** /api/2.1/unity-catalog/securable-tags/{securable_type}/{full_name} | Update tags for a securable



## securable_tagslist

> crate::models::CatalogTagSecurableAssignmentsList securable_tagslist(securable_type, full_name)
Get tags for a securable

Gets tag assignments for an entity. The caller must be either the owner of the securable, or a metastore admin, or have at least USE / SELECT privilege on the associated securable. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**securable_type** | [**serde_json::Value**](.md) | The type of the unity catalog securable entity. | [required] |
**full_name** | **String** | The fully qualified name of the unity catalog securable entity. | [required] |

### Return type

[**crate::models::CatalogTagSecurableAssignmentsList**](CatalogTagSecurableAssignmentsList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## securable_tagsupdate

> crate::models::CatalogTagSecurableAssignmentsList securable_tagsupdate(securable_type, full_name, catalog_update_tags)
Update tags for a securable

Update tag assignments for an entity The caller must be either the owner of the securable, or a metastore admin, or have at least USE / SELECT and APPLY_TAG privilege on the associated securable. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**securable_type** | [**serde_json::Value**](.md) | The type of the unity catalog securable entity. | [required] |
**full_name** | **String** | The fully qualified name of the unity catalog securable entity. | [required] |
**catalog_update_tags** | Option<[**CatalogUpdateTags**](CatalogUpdateTags.md)> |  |  |

### Return type

[**crate::models::CatalogTagSecurableAssignmentsList**](CatalogTagSecurableAssignmentsList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

