# \WorkspaceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**workspacedelete**](WorkspaceApi.md#workspacedelete) | **POST** /api/2.0/workspace/delete | Delete a workspace object
[**workspaceexport**](WorkspaceApi.md#workspaceexport) | **GET** /api/2.0/workspace/export | Export a workspace object
[**workspaceget_status**](WorkspaceApi.md#workspaceget_status) | **GET** /api/2.0/workspace/get-status | Get status
[**workspaceget_workspace_object_permission_levels**](WorkspaceApi.md#workspaceget_workspace_object_permission_levels) | **GET** /api/2.0/permissions/{workspace_object_type}/{workspace_object_id}/permissionLevels | Get workspace object permission levels
[**workspaceget_workspace_object_permissions**](WorkspaceApi.md#workspaceget_workspace_object_permissions) | **GET** /api/2.0/permissions/{workspace_object_type}/{workspace_object_id} | Get workspace object permissions
[**workspaceimport**](WorkspaceApi.md#workspaceimport) | **POST** /api/2.0/workspace/import | Import a workspace object
[**workspacelist**](WorkspaceApi.md#workspacelist) | **GET** /api/2.0/workspace/list | List contents
[**workspacemkdirs**](WorkspaceApi.md#workspacemkdirs) | **POST** /api/2.0/workspace/mkdirs | Create a directory
[**workspaceset_workspace_object_permissions**](WorkspaceApi.md#workspaceset_workspace_object_permissions) | **PUT** /api/2.0/permissions/{workspace_object_type}/{workspace_object_id} | Set workspace object permissions
[**workspaceupdate_workspace_object_permissions**](WorkspaceApi.md#workspaceupdate_workspace_object_permissions) | **PATCH** /api/2.0/permissions/{workspace_object_type}/{workspace_object_id} | Update workspace object permissions



## workspacedelete

> serde_json::Value workspacedelete(workspace_delete)
Delete a workspace object

Deletes an object or a directory (and optionally recursively deletes all objects in the directory). * If `path` does not exist, this call returns an error `RESOURCE_DOES_NOT_EXIST`. * If `path` is a non-empty directory and `recursive` is set to `false`, this call returns an error `DIRECTORY_NOT_EMPTY`.  Object deletion cannot be undone and deleting a directory recursively is not atomic. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_delete** | Option<[**WorkspaceDelete**](WorkspaceDelete.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaceexport

> crate::models::WorkspaceExportResponse workspaceexport(path, format, direct_download)
Export a workspace object

Exports an object or the contents of an entire directory.  If `path` does not exist, this call returns an error `RESOURCE_DOES_NOT_EXIST`.  If the exported data would exceed size limit, this call returns `MAX_NOTEBOOK_SIZE_EXCEEDED`. Currently, this API does not support exporting a library. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | The absolute path of the object or directory. Exporting a directory is only supported for the `DBC` and `SOURCE` format. | [required] |
**format** | Option<**String**> | This specifies the format of the exported file. By default, this is `SOURCE`.  The value is case sensitive.  - `SOURCE`: The notebook is exported as source code. - `HTML`: The notebook is exported as an HTML file. - `JUPYTER`: The notebook is exported as a Jupyter/IPython Notebook file. - `DBC`: The notebook is exported in Databricks archive format. - `R_MARKDOWN`: The notebook is exported to R Markdown format.  |  |[default to SOURCE]
**direct_download** | Option<**bool**> | Flag to enable direct download. If it is `true`, the response is the exported file itself. Otherwise, by default, the response contains content in the form of a base64 encoded string.  |  |

### Return type

[**crate::models::WorkspaceExportResponse**](WorkspaceExportResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaceget_status

> crate::models::WorkspaceObjectInfo workspaceget_status(path)
Get status

Gets the status of an object or a directory. If `path` does not exist, this call returns an error `RESOURCE_DOES_NOT_EXIST`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | The absolute path of the notebook or directory. | [required] |

### Return type

[**crate::models::WorkspaceObjectInfo**](WorkspaceObjectInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaceget_workspace_object_permission_levels

> crate::models::WorkspaceGetWorkspaceObjectPermissionLevelsResponse workspaceget_workspace_object_permission_levels(workspace_object_type, workspace_object_id)
Get workspace object permission levels

Gets the permission levels that a user can have on an object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_object_type** | [**serde_json::Value**](.md) | The workspace object type for which to get or manage permissions. | [required] |
**workspace_object_id** | [**serde_json::Value**](.md) | The workspace object for which to get or manage permissions. | [required] |

### Return type

[**crate::models::WorkspaceGetWorkspaceObjectPermissionLevelsResponse**](WorkspaceGetWorkspaceObjectPermissionLevelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaceget_workspace_object_permissions

> crate::models::WorkspaceWorkspaceObjectPermissions workspaceget_workspace_object_permissions(workspace_object_type, workspace_object_id)
Get workspace object permissions

Gets the permissions of a workspace object. Workspace objects can inherit permissions from their parent objects or root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_object_type** | **String** | The workspace object type for which to get or manage permissions. | [required] |
**workspace_object_id** | **String** | The workspace object for which to get or manage permissions. | [required] |

### Return type

[**crate::models::WorkspaceWorkspaceObjectPermissions**](WorkspaceWorkspaceObjectPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaceimport

> serde_json::Value workspaceimport(workspace_import)
Import a workspace object

Imports a workspace object (for example, a notebook or file) or the contents of an entire directory. If `path` already exists and `overwrite` is set to `false`, this call returns an error `RESOURCE_ALREADY_EXISTS`. One can only use `DBC` format to import a directory. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_import** | Option<[**WorkspaceImport**](WorkspaceImport.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspacelist

> crate::models::WorkspaceListResponse workspacelist(path, notebooks_modified_after)
List contents

Lists the contents of a directory, or the object if it is not a directory. If the input path does not exist, this call returns an error `RESOURCE_DOES_NOT_EXIST`. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | The absolute path of the notebook or directory. | [required] |
**notebooks_modified_after** | Option<**i32**> | UTC timestamp in milliseconds |  |

### Return type

[**crate::models::WorkspaceListResponse**](WorkspaceListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspacemkdirs

> serde_json::Value workspacemkdirs(workspace_mkdirs)
Create a directory

Creates the specified directory (and necessary parent directories if they do not exist).  If there is an object (not a directory) at any prefix of the input path, this call returns  an error `RESOURCE_ALREADY_EXISTS`.  Note that if this operation fails it may have succeeded in creating some of the necessary parent directories. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_mkdirs** | Option<[**WorkspaceMkdirs**](WorkspaceMkdirs.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaceset_workspace_object_permissions

> crate::models::WorkspaceWorkspaceObjectPermissions workspaceset_workspace_object_permissions(workspace_object_type, workspace_object_id, workspace_workspace_object_permissions_request)
Set workspace object permissions

Sets permissions on a workspace object. Workspace objects can inherit permissions from their parent objects or root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_object_type** | **String** | The workspace object type for which to get or manage permissions. | [required] |
**workspace_object_id** | **String** | The workspace object for which to get or manage permissions. | [required] |
**workspace_workspace_object_permissions_request** | Option<[**WorkspaceWorkspaceObjectPermissionsRequest**](WorkspaceWorkspaceObjectPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::WorkspaceWorkspaceObjectPermissions**](WorkspaceWorkspaceObjectPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaceupdate_workspace_object_permissions

> crate::models::WorkspaceWorkspaceObjectPermissions workspaceupdate_workspace_object_permissions(workspace_object_type, workspace_object_id, workspace_workspace_object_permissions_request)
Update workspace object permissions

Updates the permissions on a workspace object. Workspace objects can inherit permissions from their parent objects or root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_object_type** | **String** | The workspace object type for which to get or manage permissions. | [required] |
**workspace_object_id** | **String** | The workspace object for which to get or manage permissions. | [required] |
**workspace_workspace_object_permissions_request** | Option<[**WorkspaceWorkspaceObjectPermissionsRequest**](WorkspaceWorkspaceObjectPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::WorkspaceWorkspaceObjectPermissions**](WorkspaceWorkspaceObjectPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

