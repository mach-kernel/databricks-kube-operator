# \FunctionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**functionscreate**](FunctionsApi.md#functionscreate) | **POST** /api/2.1/unity-catalog/functions | Create a function
[**functionsdelete**](FunctionsApi.md#functionsdelete) | **DELETE** /api/2.1/unity-catalog/functions/{name} | Delete a function
[**functionsget**](FunctionsApi.md#functionsget) | **GET** /api/2.1/unity-catalog/functions/{name} | Get a function
[**functionslist**](FunctionsApi.md#functionslist) | **GET** /api/2.1/unity-catalog/functions | List functions
[**functionsupdate**](FunctionsApi.md#functionsupdate) | **PATCH** /api/2.1/unity-catalog/functions/{name} | Update a function



## functionscreate

> crate::models::CatalogFunctionInfo functionscreate(catalog_create_function)
Create a function

Creates a new function  The user must have the following permissions in order for the function to be created: - **USE_CATALOG** on the function's parent catalog - **USE_SCHEMA** and **CREATE_FUNCTION** on the function's parent schema 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_create_function** | Option<[**CatalogCreateFunction**](CatalogCreateFunction.md)> |  |  |

### Return type

[**crate::models::CatalogFunctionInfo**](CatalogFunctionInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functionsdelete

> serde_json::Value functionsdelete(name, force)
Delete a function

Deletes the function that matches the supplied name. For the deletion to succeed, the user must satisfy one of the following conditions: - Is the owner of the function's parent catalog - Is the owner of the function's parent schema and have the **USE_CATALOG** privilege on its parent catalog - Is the owner of the function itself and have both the **USE_CATALOG** privilege on its parent catalog and the **USE_SCHEMA** privilege on its parent schema 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The fully-qualified name of the function (of the form __catalog_name__.__schema_name__.__function__name__). | [required] |
**force** | Option<**bool**> | Force deletion even if the function is notempty. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functionsget

> crate::models::CatalogFunctionInfo functionsget(name)
Get a function

Gets a function from within a parent catalog and schema. For the fetch to succeed, the user must satisfy one of the following requirements: - Is a metastore admin - Is an owner of the function's parent catalog - Have the **USE_CATALOG** privilege on the function's parent catalog and be the owner of the function - Have the **USE_CATALOG** privilege on the function's parent catalog, the **USE_SCHEMA** privilege on the function's parent schema, and the **EXECUTE** privilege on the function itself 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The fully-qualified name of the function (of the form __catalog_name__.__schema_name__.__function__name__). | [required] |

### Return type

[**crate::models::CatalogFunctionInfo**](CatalogFunctionInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functionslist

> crate::models::CatalogListFunctionsResponse functionslist(catalog_name, schema_name)
List functions

List functions within the specified parent catalog and schema. If the user is a metastore admin, all functions are returned in the output list. Otherwise, the user must have the **USE_CATALOG** privilege on the catalog and the **USE_SCHEMA** privilege on the schema, and the output list contains only functions for which either the user has the **EXECUTE** privilege or the user is the owner. There is no guarantee of a specific ordering of the elements in the array. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_name** | **String** | Name of parent catalog for functions of interest. | [required] |
**schema_name** | **String** | Parent schema of functions. | [required] |

### Return type

[**crate::models::CatalogListFunctionsResponse**](CatalogListFunctionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## functionsupdate

> crate::models::CatalogFunctionInfo functionsupdate(name, catalog_update_function)
Update a function

Updates the function that matches the supplied name. Only the owner of the function can be updated. If the user is not a metastore admin, the user must be a member of the group that is the new function owner. - Is a metastore admin - Is the owner of the function's parent catalog - Is the owner of the function's parent schema and has the **USE_CATALOG** privilege on its parent catalog - Is the owner of the function itself and has the **USE_CATALOG** privilege on its parent catalog as well as the **USE_SCHEMA** privilege on the function's parent schema. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The fully-qualified name of the function (of the form __catalog_name__.__schema_name__.__function__name__). | [required] |
**catalog_update_function** | Option<[**CatalogUpdateFunction**](CatalogUpdateFunction.md)> |  |  |

### Return type

[**crate::models::CatalogFunctionInfo**](CatalogFunctionInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

