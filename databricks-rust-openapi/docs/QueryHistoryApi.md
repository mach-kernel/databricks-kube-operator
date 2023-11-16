# \QueryHistoryApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**query_historylist**](QueryHistoryApi.md#query_historylist) | **GET** /api/2.0/sql/history/queries | List Queries



## query_historylist

> crate::models::SqlListQueriesResponse query_historylist(filter_by, max_results, page_token, include_metrics)
List Queries

List the history of queries through SQL warehouses.  You can filter by user ID, warehouse ID, status, and time range. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_by** | Option<[**SqlQueryFilter**](.md)> | A filter to limit query history results. This field is optional. |  |
**max_results** | Option<**i32**> | Limit the number of results returned in one page. The default is 100. |  |
**page_token** | Option<**String**> | A token that can be used to get the next page of results. |  |
**include_metrics** | Option<**bool**> | Whether to include metrics about query. |  |

### Return type

[**crate::models::SqlListQueriesResponse**](SqlListQueriesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

