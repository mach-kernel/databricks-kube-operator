# \TablesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tablesdelete**](TablesApi.md#tablesdelete) | **DELETE** /api/2.1/unity-catalog/tables/{full_name} | Delete a table
[**tablesget**](TablesApi.md#tablesget) | **GET** /api/2.1/unity-catalog/tables/{full_name} | Get a table
[**tableslist**](TablesApi.md#tableslist) | **GET** /api/2.1/unity-catalog/tables | List tables
[**tableslist_summaries**](TablesApi.md#tableslist_summaries) | **GET** /api/2.1/unity-catalog/table-summaries | List table summaries
[**tablesupdate**](TablesApi.md#tablesupdate) | **PATCH** /api/2.1/unity-catalog/tables/{full_name} | Update a table owner.



## tablesdelete

> serde_json::Value tablesdelete(full_name)
Delete a table

Deletes a table from the specified parent catalog and schema. The caller must be the owner of the parent catalog, have the **USE_CATALOG** privilege on the parent catalog and be the owner of the parent schema, or be the owner of the table and have the **USE_CATALOG** privilege on the parent catalog and the **USE_SCHEMA** privilege on the parent schema. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name** | **String** | Full name of the table. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tablesget

> crate::models::CatalogTableInfo tablesget(full_name, include_delta_metadata)
Get a table

Gets a table from the metastore for a specific catalog and schema. The caller must be a metastore admin, be the owner of the table and have the **USE_CATALOG** privilege on the parent catalog and the **USE_SCHEMA** privilege on the parent schema, or be the owner of the table and have the **SELECT** privilege on it as well. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name** | **String** | Full name of the table. | [required] |
**include_delta_metadata** | Option<**bool**> | Whether delta metadata should be included in the response. |  |

### Return type

[**crate::models::CatalogTableInfo**](CatalogTableInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tableslist

> crate::models::CatalogListTablesResponse tableslist(catalog_name, schema_name, max_results, page_token, include_delta_metadata)
List tables

Gets an array of all tables for the current metastore under the parent catalog and schema. The caller must be a metastore admin or an owner of (or have the **SELECT** privilege on) the table. For the latter case, the caller must also be the owner or have the **USE_CATALOG** privilege on the parent catalog and the **USE_SCHEMA** privilege on the parent schema. There is no guarantee of a specific ordering of the elements in the array. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_name** | **String** | Name of parent catalog for tables of interest. | [required] |
**schema_name** | **String** | Parent schema of tables. | [required] |
**max_results** | Option<**i32**> | Maximum number of tables to return (page length). If not set, all accessible tables in the schema are returned. If set to:    * greater than 0, page length is the minimum of this value and a server configured value.   * equal to 0, page length is set to a server configured value.   * lesser than 0, invalid parameter error.   |  |
**page_token** | Option<**String**> | Opaque token to send for the next page of results (pagination). |  |
**include_delta_metadata** | Option<**bool**> | Whether delta metadata should be included in the response. |  |

### Return type

[**crate::models::CatalogListTablesResponse**](CatalogListTablesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tableslist_summaries

> crate::models::CatalogListTableSummariesResponse tableslist_summaries(catalog_name, schema_name_pattern, table_name_pattern, max_results, page_token)
List table summaries

Gets an array of summaries for tables for a schema and catalog within the metastore. The table summaries returned are either:  * summaries for all tables (within the current metastore and parent catalog and schema), when the user is a metastore admin, or: * summaries for all tables and schemas (within the current metastore and parent catalog)   for which the user has ownership or the **SELECT** privilege on the table and ownership or **USE_SCHEMA** privilege on the schema,   provided that the user also has ownership or the **USE_CATALOG** privilege on the parent catalog.   There is no guarantee of a specific ordering of the elements in the array. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_name** | **String** | Name of parent catalog for tables of interest. | [required] |
**schema_name_pattern** | Option<**String**> | A sql LIKE pattern (% and _) for schema names. All schemas will be returned if not set or empty. |  |
**table_name_pattern** | Option<**String**> | A sql LIKE pattern (% and _) for table names. All tables will be returned if not set or empty. |  |
**max_results** | Option<**i32**> | Maximum number of tables to return (page length). Defaults to 10000. |  |
**page_token** | Option<**String**> | Opaque token to send for the next page of results (pagination). |  |

### Return type

[**crate::models::CatalogListTableSummariesResponse**](CatalogListTableSummariesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tablesupdate

> serde_json::Value tablesupdate(full_name, tablesupdate_request)
Update a table owner.

Change the owner of the table. The caller must be the owner of the parent catalog, have the **USE_CATALOG** privilege on the parent catalog and be the owner of the parent schema, or be the owner of the table and have the **USE_CATALOG** privilege on the parent catalog and the **USE_SCHEMA** privilege on the parent schema. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name** | **String** | Full name of the table. | [required] |
**tablesupdate_request** | Option<[**TablesupdateRequest**](TablesupdateRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

