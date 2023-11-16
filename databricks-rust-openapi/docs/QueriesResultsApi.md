# \QueriesResultsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**queriescreate**](QueriesResultsApi.md#queriescreate) | **POST** /api/2.0/preview/sql/queries | Create a new query definition
[**queriesdelete**](QueriesResultsApi.md#queriesdelete) | **DELETE** /api/2.0/preview/sql/queries/{query_id} | Delete a query
[**queriesget**](QueriesResultsApi.md#queriesget) | **GET** /api/2.0/preview/sql/queries/{query_id} | Get a query definition.
[**querieslist**](QueriesResultsApi.md#querieslist) | **GET** /api/2.0/preview/sql/queries | Get a list of queries
[**queriesrestore**](QueriesResultsApi.md#queriesrestore) | **POST** /api/2.0/preview/sql/queries/trash/{query_id} | Restore a query
[**queriesupdate**](QueriesResultsApi.md#queriesupdate) | **POST** /api/2.0/preview/sql/queries/{query_id} | Change a query definition



## queriescreate

> crate::models::SqlQuery queriescreate(sql_query_post_content)
Create a new query definition

Creates a new query definition. Queries created with this endpoint belong to the authenticated user making the request.  The `data_source_id` field specifies the ID of the SQL warehouse to run this query against. You can use the Data Sources API to see a complete list of available SQL warehouses. Or you can copy the `data_source_id` from an existing query.  **Note**: You cannot add a visualization until you create the query. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sql_query_post_content** | [**SqlQueryPostContent**](SqlQueryPostContent.md) | A query object to be created. | [required] |

### Return type

[**crate::models::SqlQuery**](SqlQuery.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## queriesdelete

> serde_json::Value queriesdelete(query_id)
Delete a query

Moves a query to the trash. Trashed queries immediately disappear from searches and list views, and they cannot be used for alerts. The trash is deleted after 30 days. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## queriesget

> crate::models::SqlQuery queriesget(query_id)
Get a query definition.

Retrieve a query object definition along with contextual permissions information about the currently authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**crate::models::SqlQuery**](SqlQuery.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## querieslist

> crate::models::SqlQueryList querieslist(page_size, page, order, q)
Get a list of queries

Gets a list of queries. Optionally, this list can be filtered by a search term. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Number of queries to return per page. |  |
**page** | Option<**i32**> | Page number to retrieve. |  |
**order** | Option<**String**> | Name of query attribute to order by. Default sort order is ascending. Append a dash (`-`) to order descending instead.   - `name`: The name of the query.   - `created_at`: The timestamp the query was created.   - `runtime`: The time it took to run this query. This is blank for parameterized queries. A blank value is treated as the highest value for sorting.   - `executed_at`: The timestamp when the query was last run.   - `created_by`: The user name of the user that created the query.    |  |
**q** | Option<**String**> | Full text search term |  |

### Return type

[**crate::models::SqlQueryList**](SqlQueryList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## queriesrestore

> serde_json::Value queriesrestore(query_id)
Restore a query

Restore a query that has been moved to the trash. A restored query appears in list views and searches. You can use restored queries for alerts. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_id** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## queriesupdate

> crate::models::SqlQuery queriesupdate(query_id, sql_query_edit_content)
Change a query definition

Modify this query definition.  **Note**: You cannot undo this operation. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_id** | [**serde_json::Value**](.md) |  | [required] |
**sql_query_edit_content** | [**SqlQueryEditContent**](SqlQueryEditContent.md) | The query definition that will replace the current definition for this `query_id`. | [required] |

### Return type

[**crate::models::SqlQuery**](SqlQuery.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

