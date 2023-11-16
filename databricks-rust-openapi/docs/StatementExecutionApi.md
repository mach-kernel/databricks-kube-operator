# \StatementExecutionApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**statement_executioncancel_execution**](StatementExecutionApi.md#statement_executioncancel_execution) | **POST** /api/2.0/sql/statements/{statement_id}/cancel | Cancel statement execution
[**statement_executionexecute_statement**](StatementExecutionApi.md#statement_executionexecute_statement) | **POST** /api/2.0/sql/statements/ | Execute a SQL statement
[**statement_executionget_statement**](StatementExecutionApi.md#statement_executionget_statement) | **GET** /api/2.0/sql/statements/{statement_id} | Get status, manifest, and result first chunk
[**statement_executionget_statement_result_chunk_n**](StatementExecutionApi.md#statement_executionget_statement_result_chunk_n) | **GET** /api/2.0/sql/statements/{statement_id}/result/chunks/{chunk_index} | Get result chunk by index



## statement_executioncancel_execution

> statement_executioncancel_execution(statement_id, statement_id2)
Cancel statement execution

Requests that an executing statement be canceled. Callers must poll for status to see the terminal state. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**statement_id** | **String** |  | [required] |
**statement_id2** | [**serde_json::Value**](.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statement_executionexecute_statement

> crate::models::StatementExecutiongetStatement200Response statement_executionexecute_statement(sql_execute_statement_request)
Execute a SQL statement

Execute a SQL statement, and if flagged as such, await its result for a specified time. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sql_execute_statement_request** | [**SqlExecuteStatementRequest**](SqlExecuteStatementRequest.md) |  | [required] |

### Return type

[**crate::models::StatementExecutiongetStatement200Response**](StatementExecutiongetStatement_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statement_executionget_statement

> crate::models::StatementExecutiongetStatement200Response statement_executionget_statement(statement_id, statement_id2)
Get status, manifest, and result first chunk

This request can be used to poll for the statement's status. When the `Statusstate` field is `SUCCEEDED` it will also return the result manifest and the first chunk of the result data. When the statement is in the terminal states `CANCELED`, `CLOSED` or `FAILED`, it returns HTTP 200 with the state set. After at least 12 hours in terminal state, the statement is removed from the warehouse and further calls will receive an HTTP 404 response.  **NOTE** This call currently may take up to 5 seconds to get the latest status and result. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**statement_id** | **String** |  | [required] |
**statement_id2** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**crate::models::StatementExecutiongetStatement200Response**](StatementExecutiongetStatement_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statement_executionget_statement_result_chunk_n

> crate::models::SqlResultData statement_executionget_statement_result_chunk_n(statement_id, chunk_index, statement_id2, chunk_index2)
Get result chunk by index

After the statement execution has `SUCCEEDED`, the result data can be fetched by chunks. Whereas the first chuck with `chunk_index=0` is typically fetched through a `get status` request, subsequent chunks can be fetched using a `get result` request. The response structure is identical to the nested `result` element described in the `get status` request, and similarly includes the `next_chunk_index` and `next_chunk_internal_link` fields for simple iteration through the result set. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**statement_id** | **String** |  | [required] |
**chunk_index** | **String** |  | [required] |
**statement_id2** | [**serde_json::Value**](.md) |  | [required] |
**chunk_index2** | [**serde_json::Value**](.md) |  | [required] |

### Return type

[**crate::models::SqlResultData**](SqlResultData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

