# \MetastoresApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**metastoresassign**](MetastoresApi.md#metastoresassign) | **PUT** /api/2.1/unity-catalog/workspaces/{workspace_id}/metastore | Create an assignment
[**metastorescreate**](MetastoresApi.md#metastorescreate) | **POST** /api/2.1/unity-catalog/metastores | Create a metastore
[**metastorescurrent**](MetastoresApi.md#metastorescurrent) | **GET** /api/2.1/unity-catalog/current-metastore-assignment | Get metastore assignment for workspace
[**metastoresdelete**](MetastoresApi.md#metastoresdelete) | **DELETE** /api/2.1/unity-catalog/metastores/{id} | Delete a metastore
[**metastoresenable_optimization**](MetastoresApi.md#metastoresenable_optimization) | **PATCH** /api/2.0/predictive-optimization/service | Toggle predictive optimization on the metastore
[**metastoresget**](MetastoresApi.md#metastoresget) | **GET** /api/2.1/unity-catalog/metastores/{id} | Get a metastore
[**metastoreslist**](MetastoresApi.md#metastoreslist) | **GET** /api/2.1/unity-catalog/metastores | List metastores
[**metastoressummary**](MetastoresApi.md#metastoressummary) | **GET** /api/2.1/unity-catalog/metastore_summary | Get a metastore summary
[**metastoresunassign**](MetastoresApi.md#metastoresunassign) | **DELETE** /api/2.1/unity-catalog/workspaces/{workspace_id}/metastore | Delete an assignment
[**metastoresupdate**](MetastoresApi.md#metastoresupdate) | **PATCH** /api/2.1/unity-catalog/metastores/{id} | Update a metastore
[**metastoresupdate_assignment**](MetastoresApi.md#metastoresupdate_assignment) | **PATCH** /api/2.1/unity-catalog/workspaces/{workspace_id}/metastore | Update an assignment



## metastoresassign

> serde_json::Value metastoresassign(workspace_id, catalog_create_metastore_assignment)
Create an assignment

Creates a new metastore assignment. If an assignment for the same __workspace_id__ exists, it will be overwritten by the new __metastore_id__ and __default_catalog_name__. The caller must be an account admin. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **i64** | A workspace ID. | [required] |
**catalog_create_metastore_assignment** | Option<[**CatalogCreateMetastoreAssignment**](CatalogCreateMetastoreAssignment.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metastorescreate

> crate::models::CatalogMetastoreInfo metastorescreate(catalog_create_metastore)
Create a metastore

Creates a new metastore based on a provided name and storage root path.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_create_metastore** | Option<[**CatalogCreateMetastore**](CatalogCreateMetastore.md)> |  |  |

### Return type

[**crate::models::CatalogMetastoreInfo**](CatalogMetastoreInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metastorescurrent

> crate::models::CatalogMetastoreAssignment metastorescurrent()
Get metastore assignment for workspace

Gets the metastore assignment for the workspace being accessed. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CatalogMetastoreAssignment**](CatalogMetastoreAssignment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metastoresdelete

> serde_json::Value metastoresdelete(id, force)
Delete a metastore

Deletes a metastore. The caller must be a metastore admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique ID of the metastore. | [required] |
**force** | Option<**bool**> | Force deletion even if the metastore is not empty. Default is false. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metastoresenable_optimization

> crate::models::CatalogUpdatePredictiveOptimizationResponse metastoresenable_optimization(catalog_update_predictive_optimization)
Toggle predictive optimization on the metastore

Enables or disables predictive optimization on the metastore. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_update_predictive_optimization** | Option<[**CatalogUpdatePredictiveOptimization**](CatalogUpdatePredictiveOptimization.md)> |  |  |

### Return type

[**crate::models::CatalogUpdatePredictiveOptimizationResponse**](CatalogUpdatePredictiveOptimizationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metastoresget

> crate::models::CatalogMetastoreInfo metastoresget(id)
Get a metastore

Gets a metastore that matches the supplied ID. The caller must be a metastore admin to retrieve this info.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique ID of the metastore. | [required] |

### Return type

[**crate::models::CatalogMetastoreInfo**](CatalogMetastoreInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metastoreslist

> crate::models::CatalogListMetastoresResponse metastoreslist()
List metastores

Gets an array of the available metastores (as __MetastoreInfo__ objects). The caller must be an admin to retrieve this info. There is no guarantee of a specific ordering of the elements in the array. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CatalogListMetastoresResponse**](CatalogListMetastoresResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metastoressummary

> crate::models::CatalogGetMetastoreSummaryResponse metastoressummary()
Get a metastore summary

Gets information about a metastore. This summary includes the storage credential, the cloud vendor, the cloud region, and the global metastore ID. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CatalogGetMetastoreSummaryResponse**](CatalogGetMetastoreSummaryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metastoresunassign

> serde_json::Value metastoresunassign(metastore_id, workspace_id)
Delete an assignment

Deletes a metastore assignment. The caller must be an account administrator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metastore_id** | **String** | Query for the ID of the metastore to delete. | [required] |
**workspace_id** | **i64** | A workspace ID. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metastoresupdate

> crate::models::CatalogMetastoreInfo metastoresupdate(id, catalog_update_metastore)
Update a metastore

Updates information for a specific metastore. The caller must be a metastore admin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique ID of the metastore. | [required] |
**catalog_update_metastore** | Option<[**CatalogUpdateMetastore**](CatalogUpdateMetastore.md)> |  |  |

### Return type

[**crate::models::CatalogMetastoreInfo**](CatalogMetastoreInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metastoresupdate_assignment

> serde_json::Value metastoresupdate_assignment(workspace_id, catalog_update_metastore_assignment)
Update an assignment

Updates a metastore assignment. This operation can be used to update __metastore_id__ or __default_catalog_name__ for a specified Workspace, if the Workspace is already assigned a metastore. The caller must be an account admin to update __metastore_id__; otherwise, the caller can be a Workspace admin. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **i64** | A workspace ID. | [required] |
**catalog_update_metastore_assignment** | Option<[**CatalogUpdateMetastoreAssignment**](CatalogUpdateMetastoreAssignment.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

