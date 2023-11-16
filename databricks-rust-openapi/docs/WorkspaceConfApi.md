# \WorkspaceConfApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**workspace_confget_status**](WorkspaceConfApi.md#workspace_confget_status) | **GET** /api/2.0/workspace-conf | Check configuration status
[**workspace_confset_status**](WorkspaceConfApi.md#workspace_confset_status) | **PATCH** /api/2.0/workspace-conf | Enable/disable features



## workspace_confget_status

> ::std::collections::HashMap<String, String> workspace_confget_status(keys)
Check configuration status

Gets the configuration status for a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**keys** | **String** |  | [required] |

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_confset_status

> workspace_confset_status(request_body)
Enable/disable features

Sets the configuration status for a workspace, including enabling or disabling it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | Option<[**::std::collections::HashMap<String, String>**](String.md)> |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

