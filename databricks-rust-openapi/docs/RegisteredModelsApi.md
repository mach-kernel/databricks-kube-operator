# \RegisteredModelsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**registered_modelscreate**](RegisteredModelsApi.md#registered_modelscreate) | **POST** /api/2.1/unity-catalog/models | Create a Registered Model
[**registered_modelsdelete**](RegisteredModelsApi.md#registered_modelsdelete) | **DELETE** /api/2.1/unity-catalog/models/{full_name} | Delete a Registered Model
[**registered_modelsget**](RegisteredModelsApi.md#registered_modelsget) | **GET** /api/2.1/unity-catalog/models/{full_name} | Get a Registered Model
[**registered_modelslist**](RegisteredModelsApi.md#registered_modelslist) | **GET** /api/2.1/unity-catalog/models | List Registered Models
[**registered_modelsupdate**](RegisteredModelsApi.md#registered_modelsupdate) | **PATCH** /api/2.1/unity-catalog/models/{full_name} | Update a Registered Model



## registered_modelscreate

> crate::models::CatalogRegisteredModelInfo registered_modelscreate(catalog_create_registered_model_request)
Create a Registered Model

Creates a new registered model in Unity Catalog.  File storage for model versions in the registered model will be located in the default location which is specified by the parent schema, or the parent catalog, or the Metastore.  For registered model creation to succeed, the user must satisfy the following conditions: - The caller must be a metastore admin, or be the owner of the parent catalog and schema,   or have the **USE_CATALOG** privilege on the parent catalog   and the **USE_SCHEMA** privilege on the parent schema. - The caller must have the **CREATE MODEL** or **CREATE FUNCTION** privilege on the parent schema. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_create_registered_model_request** | [**CatalogCreateRegisteredModelRequest**](CatalogCreateRegisteredModelRequest.md) |  | [required] |

### Return type

[**crate::models::CatalogRegisteredModelInfo**](CatalogRegisteredModelInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registered_modelsdelete

> registered_modelsdelete(full_name)
Delete a Registered Model

Deletes a registered model and all its model versions from the specified parent catalog and schema.  The caller must be a metastore admin or an owner of the registered model. For the latter case, the caller must also be the owner or have the **USE_CATALOG** privilege on the parent catalog and the **USE_SCHEMA** privilege on the parent schema. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name** | **String** | The three-level (fully qualified) name of the registered model | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registered_modelsget

> crate::models::CatalogRegisteredModelInfo registered_modelsget(full_name)
Get a Registered Model

Get a registered model.  The caller must be a metastore admin or an owner of (or have the **EXECUTE** privilege on) the registered model. For the latter case, the caller must also be the owner or have the **USE_CATALOG** privilege on the parent catalog and the **USE_SCHEMA** privilege on the parent schema. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name** | **String** | The three-level (fully qualified) name of the registered model | [required] |

### Return type

[**crate::models::CatalogRegisteredModelInfo**](CatalogRegisteredModelInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registered_modelslist

> crate::models::CatalogListRegisteredModelsResponse registered_modelslist(catalog_name, schema_name, max_results, page_token)
List Registered Models

List registered models. You can list registered models under a particular schema, or list all registered models in the current metastore.  The returned models are filtered based on the privileges of the calling user. For example, the metastore admin is able to list all the registered models. A regular user needs to be the owner or have the **EXECUTE** privilege on the registered model to recieve the registered models in the response. For the latter case, the caller must also be the owner or have the **USE_CATALOG** privilege on the parent catalog and the **USE_SCHEMA** privilege on the parent schema.  There is no guarantee of a specific ordering of the elements in the response. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_name** | Option<**String**> | The identifier of the catalog under which to list registered models.             If specified, schema_name must be specified. |  |
**schema_name** | Option<**String**> | The identifier of the schema under which to list registered models.             If specified, catalog_name must be specified. |  |
**max_results** | Option<**i32**> | Max number of registered models to return. If catalog and schema are unspecified, max_results             must be specified. If max_results is unspecified, we return all results, starting from the page specified             by page_token. |  |
**page_token** | Option<**String**> | Opaque token to send for the next page of results (pagination). |  |

### Return type

[**crate::models::CatalogListRegisteredModelsResponse**](CatalogListRegisteredModelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## registered_modelsupdate

> crate::models::CatalogRegisteredModelInfo registered_modelsupdate(full_name, catalog_update_registered_model_request)
Update a Registered Model

Updates the specified registered model.  The caller must be a metastore admin or an owner of the registered model. For the latter case, the caller must also be the owner or have the **USE_CATALOG** privilege on the parent catalog and the **USE_SCHEMA** privilege on the parent schema.  Currently only the name, the owner or the comment of the registered model can be updated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name** | **String** | The three-level (fully qualified) name of the registered model | [required] |
**catalog_update_registered_model_request** | Option<[**CatalogUpdateRegisteredModelRequest**](CatalogUpdateRegisteredModelRequest.md)> |  |  |

### Return type

[**crate::models::CatalogRegisteredModelInfo**](CatalogRegisteredModelInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

