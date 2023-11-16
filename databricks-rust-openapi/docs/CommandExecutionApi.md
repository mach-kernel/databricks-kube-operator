# \CommandExecutionApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**command_executioncancel**](CommandExecutionApi.md#command_executioncancel) | **POST** /api/1.2/commands/cancel | Cancel a command
[**command_executioncommand_status**](CommandExecutionApi.md#command_executioncommand_status) | **GET** /api/1.2/commands/status | Get command info
[**command_executioncontext_status**](CommandExecutionApi.md#command_executioncontext_status) | **GET** /api/1.2/contexts/status | Get status
[**command_executioncreate**](CommandExecutionApi.md#command_executioncreate) | **POST** /api/1.2/contexts/create | Create an execution context
[**command_executiondestroy**](CommandExecutionApi.md#command_executiondestroy) | **POST** /api/1.2/contexts/destroy | Delete an execution context
[**command_executionexecute**](CommandExecutionApi.md#command_executionexecute) | **POST** /api/1.2/commands/execute | Run a command



## command_executioncancel

> serde_json::Value command_executioncancel(compute_cancel_command)
Cancel a command

Cancels a currently running command within an execution context.  The command ID is obtained from a prior successful call to __execute__.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_cancel_command** | [**ComputeCancelCommand**](ComputeCancelCommand.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## command_executioncommand_status

> crate::models::ComputeCommandStatusResponse command_executioncommand_status(cluster_id, context_id, command_id)
Get command info

Gets the status of and, if available, the results from a currently executing command.  The command ID is obtained from a prior successful call to __execute__. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **String** |  | [required] |
**context_id** | **String** |  | [required] |
**command_id** | **String** |  | [required] |

### Return type

[**crate::models::ComputeCommandStatusResponse**](ComputeCommandStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## command_executioncontext_status

> crate::models::ComputeContextStatusResponse command_executioncontext_status(cluster_id, context_id)
Get status

Gets the status for an execution context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **String** |  | [required] |
**context_id** | **String** |  | [required] |

### Return type

[**crate::models::ComputeContextStatusResponse**](ComputeContextStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## command_executioncreate

> crate::models::ComputeCreated command_executioncreate(compute_create_context)
Create an execution context

Creates an execution context for running cluster commands.  If successful, this method returns the ID of the new execution context. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_create_context** | [**ComputeCreateContext**](ComputeCreateContext.md) |  | [required] |

### Return type

[**crate::models::ComputeCreated**](ComputeCreated.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## command_executiondestroy

> serde_json::Value command_executiondestroy(compute_destroy_context)
Delete an execution context

Deletes an execution context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_destroy_context** | [**ComputeDestroyContext**](ComputeDestroyContext.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## command_executionexecute

> crate::models::ComputeCreated command_executionexecute(compute_command)
Run a command

Runs a cluster command in the given execution context, using the provided language.  If successful, it returns an ID for tracking the status of the command's execution. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_command** | [**ComputeCommand**](ComputeCommand.md) |  | [required] |

### Return type

[**crate::models::ComputeCreated**](ComputeCreated.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

