# \SubentityTagsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**subentity_tagslist**](SubentityTagsApi.md#subentity_tagslist) | **GET** /api/2.1/unity-catalog/subentity-tags/{securable_type}/{full_name}/{subentity_name} | Get tags for a subentity
[**subentity_tagsupdate**](SubentityTagsApi.md#subentity_tagsupdate) | **PATCH** /api/2.1/unity-catalog/subentity-tags/{securable_type}/{full_name}/{subentity_name} | Update tags for a subentity



## subentity_tagslist

> crate::models::CatalogTagSubentityAssignmentsList subentity_tagslist(securable_type, full_name, subentity_name)
Get tags for a subentity

Gets tag assignments for a subentity associated with a securable entity. Eg. column of a table The caller must be either the owner of the securable, or a metastore admin, or have at least USE / SELECT privilege on the associated securable. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**securable_type** | [**serde_json::Value**](.md) | The type of the unity catalog securable entity. | [required] |
**full_name** | [**serde_json::Value**](.md) | The fully qualified name of the unity catalog securable entity. | [required] |
**subentity_name** | **String** | The name of subentity associated with the securable entity | [required] |

### Return type

[**crate::models::CatalogTagSubentityAssignmentsList**](CatalogTagSubentityAssignmentsList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subentity_tagsupdate

> crate::models::CatalogTagSubentityAssignmentsList subentity_tagsupdate(securable_type, full_name, subentity_name, catalog_update_tags)
Update tags for a subentity

Update tag assignments for a subentity associated with a securable entity. The caller must be either the owner of the securable, or a metastore admin, or have at least USE / SELECT and APPLY_TAG privilege on the associated securable. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**securable_type** | [**serde_json::Value**](.md) | The type of the unity catalog securable entity. | [required] |
**full_name** | [**serde_json::Value**](.md) | The fully qualified name of the unity catalog securable entity. | [required] |
**subentity_name** | **String** | The name of subentity associated with the securable entity | [required] |
**catalog_update_tags** | Option<[**CatalogUpdateTags**](CatalogUpdateTags.md)> |  |  |

### Return type

[**crate::models::CatalogTagSubentityAssignmentsList**](CatalogTagSubentityAssignmentsList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

