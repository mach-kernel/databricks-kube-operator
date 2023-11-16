# \SchemasApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**schemascreate**](SchemasApi.md#schemascreate) | **POST** /api/2.1/unity-catalog/schemas | Create a schema
[**schemasdelete**](SchemasApi.md#schemasdelete) | **DELETE** /api/2.1/unity-catalog/schemas/{full_name} | Delete a schema
[**schemasget**](SchemasApi.md#schemasget) | **GET** /api/2.1/unity-catalog/schemas/{full_name} | Get a schema
[**schemaslist**](SchemasApi.md#schemaslist) | **GET** /api/2.1/unity-catalog/schemas | List schemas
[**schemasupdate**](SchemasApi.md#schemasupdate) | **PATCH** /api/2.1/unity-catalog/schemas/{full_name} | Update a schema



## schemascreate

> crate::models::CatalogSchemaInfo schemascreate(catalog_create_schema)
Create a schema

Creates a new schema for catalog in the Metatastore. The caller must be a metastore admin, or have the **CREATE_SCHEMA** privilege in the parent catalog. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_create_schema** | Option<[**CatalogCreateSchema**](CatalogCreateSchema.md)> |  |  |

### Return type

[**crate::models::CatalogSchemaInfo**](CatalogSchemaInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemasdelete

> serde_json::Value schemasdelete(full_name)
Delete a schema

Deletes the specified schema from the parent catalog. The caller must be the owner of the schema or an owner of the parent catalog. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name** | **String** | Full name of the schema. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemasget

> crate::models::CatalogSchemaInfo schemasget(full_name)
Get a schema

Gets the specified schema within the metastore. The caller must be a metastore admin, the owner of the schema, or a user that has the **USE_SCHEMA** privilege on the schema. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name** | **String** | Full name of the schema. | [required] |

### Return type

[**crate::models::CatalogSchemaInfo**](CatalogSchemaInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemaslist

> crate::models::CatalogListSchemasResponse schemaslist(catalog_name)
List schemas

Gets an array of schemas for a catalog in the metastore. If the caller is the metastore admin or the owner of the parent catalog, all schemas for the catalog will be retrieved. Otherwise, only schemas owned by the caller (or for which the caller has the **USE_SCHEMA** privilege) will be retrieved. There is no guarantee of a specific ordering of the elements in the array. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_name** | **String** | Parent catalog for schemas of interest. | [required] |

### Return type

[**crate::models::CatalogListSchemasResponse**](CatalogListSchemasResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schemasupdate

> crate::models::CatalogSchemaInfo schemasupdate(full_name, catalog_update_schema)
Update a schema

Updates a schema for a catalog. The caller must be the owner of the schema or a metastore admin. If the caller is a metastore admin, only the __owner__ field can be changed in the update. If the __name__ field must be updated, the caller must be a metastore admin or have the **CREATE_SCHEMA** privilege on the parent catalog. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name** | **String** | Full name of the schema. | [required] |
**catalog_update_schema** | Option<[**CatalogUpdateSchema**](CatalogUpdateSchema.md)> |  |  |

### Return type

[**crate::models::CatalogSchemaInfo**](CatalogSchemaInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

