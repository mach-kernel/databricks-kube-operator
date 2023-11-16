# \GlobalInitScriptsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**global_init_scriptscreate**](GlobalInitScriptsApi.md#global_init_scriptscreate) | **POST** /api/2.0/global-init-scripts | Create init script
[**global_init_scriptsdelete**](GlobalInitScriptsApi.md#global_init_scriptsdelete) | **DELETE** /api/2.0/global-init-scripts/{script_id} | Delete init script
[**global_init_scriptsget**](GlobalInitScriptsApi.md#global_init_scriptsget) | **GET** /api/2.0/global-init-scripts/{script_id} | Get an init script
[**global_init_scriptslist**](GlobalInitScriptsApi.md#global_init_scriptslist) | **GET** /api/2.0/global-init-scripts | Get init scripts
[**global_init_scriptsupdate**](GlobalInitScriptsApi.md#global_init_scriptsupdate) | **PATCH** /api/2.0/global-init-scripts/{script_id} | Update init script



## global_init_scriptscreate

> crate::models::GlobalInitScriptscreate200Response global_init_scriptscreate(compute_global_init_script_create_request)
Create init script

Creates a new global init script in this workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_global_init_script_create_request** | [**ComputeGlobalInitScriptCreateRequest**](ComputeGlobalInitScriptCreateRequest.md) |  | [required] |

### Return type

[**crate::models::GlobalInitScriptscreate200Response**](GlobalInitScriptscreate_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## global_init_scriptsdelete

> global_init_scriptsdelete(script_id)
Delete init script

Deletes a global init script.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**script_id** | **String** | The ID of the global init script. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## global_init_scriptsget

> crate::models::ComputeGlobalInitScriptDetailsWithContent global_init_scriptsget(script_id)
Get an init script

Gets all the details of a script, including its Base64-encoded contents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**script_id** | **String** | The ID of the global init script. | [required] |

### Return type

[**crate::models::ComputeGlobalInitScriptDetailsWithContent**](ComputeGlobalInitScriptDetailsWithContent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## global_init_scriptslist

> crate::models::ComputeListGlobalInitScriptsResponse global_init_scriptslist()
Get init scripts

Get a list of all global init scripts for this workspace. This returns all properties for each script but **not** the script contents. To retrieve the contents of a script, use the [get a global init script](#operation/get-script) operation. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ComputeListGlobalInitScriptsResponse**](ComputeListGlobalInitScriptsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## global_init_scriptsupdate

> global_init_scriptsupdate(script_id, compute_global_init_script_update_request)
Update init script

Updates a global init script, specifying only the fields to change. All fields are optional.  Unspecified fields retain their current value. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**script_id** | **String** | The ID of the global init script. | [required] |
**compute_global_init_script_update_request** | [**ComputeGlobalInitScriptUpdateRequest**](ComputeGlobalInitScriptUpdateRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

