# \SystemSchemasApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**system_schemasdisable**](SystemSchemasApi.md#system_schemasdisable) | **DELETE** /api/2.1/unity-catalog/metastores/{metastore_id}/systemschemas/{schema_name} | Disable a system schema
[**system_schemasenable**](SystemSchemasApi.md#system_schemasenable) | **PUT** /api/2.1/unity-catalog/metastores/{metastore_id}/systemschemas/{schema_name} | Enable a system schema
[**system_schemaslist**](SystemSchemasApi.md#system_schemaslist) | **GET** /api/2.1/unity-catalog/metastores/{metastore_id}/systemschemas | List system schemas



## system_schemasdisable

> system_schemasdisable(metastore_id, schema_name)
Disable a system schema

Disables the system schema and removes it from the system catalog. The caller must be an account admin or a metastore admin. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metastore_id** | **String** | The metastore ID under which the system schema lives. | [required] |
**schema_name** | [**serde_json::Value**](.md) | Full name of the system schema. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_schemasenable

> system_schemasenable(metastore_id, schema_name)
Enable a system schema

Enables the system schema and adds it to the system catalog. The caller must be an account admin or a metastore admin. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metastore_id** | **String** | The metastore ID under which the system schema lives. | [required] |
**schema_name** | [**serde_json::Value**](.md) | Full name of the system schema. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## system_schemaslist

> crate::models::CatalogListSystemSchemasResponse system_schemaslist(metastore_id)
List system schemas

Gets an array of system schemas for a metastore. The caller must be an account admin or a metastore admin. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metastore_id** | **String** | The ID for the metastore in which the system schema resides. | [required] |

### Return type

[**crate::models::CatalogListSystemSchemasResponse**](CatalogListSystemSchemasResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

