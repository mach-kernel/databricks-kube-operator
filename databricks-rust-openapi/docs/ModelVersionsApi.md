# \ModelVersionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**model_versionsdelete**](ModelVersionsApi.md#model_versionsdelete) | **DELETE** /api/2.1/unity-catalog/models/{full_name}/versions/{version} | Delete a Model Version
[**model_versionsget**](ModelVersionsApi.md#model_versionsget) | **GET** /api/2.1/unity-catalog/models/{full_name}/versions/{version} | Get a Model Version
[**model_versionslist**](ModelVersionsApi.md#model_versionslist) | **GET** /api/2.1/unity-catalog/models/{full_name}/versions | List Model Versions
[**model_versionsupdate**](ModelVersionsApi.md#model_versionsupdate) | **PATCH** /api/2.1/unity-catalog/models/{full_name}/versions/{version} | Update a Model Version



## model_versionsdelete

> model_versionsdelete(full_name, version)
Delete a Model Version

Deletes a model version from the specified registered model. Any aliases assigned to the model version will also be deleted.  The caller must be a metastore admin or an owner of the parent registered model. For the latter case, the caller must also be the owner or have the **USE_CATALOG** privilege on the parent catalog and the **USE_SCHEMA** privilege on the parent schema. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name** | **String** | The three-level (fully qualified) name of the model version | [required] |
**version** | **i32** | The integer version number of the model version | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_versionsget

> crate::models::CatalogRegisteredModelInfo model_versionsget(full_name, version)
Get a Model Version

Get a model version.  The caller must be a metastore admin or an owner of (or have the **EXECUTE** privilege on) the parent registered model. For the latter case, the caller must also be the owner or have the **USE_CATALOG** privilege on the parent catalog and the **USE_SCHEMA** privilege on the parent schema. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name** | **String** | The three-level (fully qualified) name of the model version | [required] |
**version** | **i32** | The integer version number of the model version | [required] |

### Return type

[**crate::models::CatalogRegisteredModelInfo**](CatalogRegisteredModelInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_versionslist

> crate::models::CatalogListModelVersionsResponse model_versionslist(full_name, max_results, page_token)
List Model Versions

List model versions. You can list model versions under a particular schema, or list all model versions in the current metastore.  The returned models are filtered based on the privileges of the calling user. For example, the metastore admin is able to list all the model versions. A regular user needs to be the owner or have the **EXECUTE** privilege on the parent registered model to recieve the model versions in the response. For the latter case, the caller must also be the owner or have the **USE_CATALOG** privilege on the parent catalog and the **USE_SCHEMA** privilege on the parent schema.  There is no guarantee of a specific ordering of the elements in the response. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name** | **String** | The full three-level name of the registered model under which to list model versions | [required] |
**max_results** | Option<**i32**> | Max number of model versions to return |  |[default to 100]
**page_token** | Option<**String**> | Opaque token to send for the next page of results (pagination). |  |

### Return type

[**crate::models::CatalogListModelVersionsResponse**](CatalogListModelVersionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_versionsupdate

> crate::models::CatalogModelVersionInfo model_versionsupdate(full_name, version, catalog_update_model_version_request)
Update a Model Version

Updates the specified model version.  The caller must be a metastore admin or an owner of the parent registered model. For the latter case, the caller must also be the owner or have the **USE_CATALOG** privilege on the parent catalog and the **USE_SCHEMA** privilege on the parent schema.  Currently only the comment of the model version can be updated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name** | **String** | The three-level (fully qualified) name of the model version | [required] |
**version** | **i32** | The integer version number of the model version | [required] |
**catalog_update_model_version_request** | Option<[**CatalogUpdateModelVersionRequest**](CatalogUpdateModelVersionRequest.md)> |  |  |

### Return type

[**crate::models::CatalogModelVersionInfo**](CatalogModelVersionInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

