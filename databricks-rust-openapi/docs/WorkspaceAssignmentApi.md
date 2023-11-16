# \WorkspaceAssignmentApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**workspace_assignmentdelete**](WorkspaceAssignmentApi.md#workspace_assignmentdelete) | **DELETE** /api/2.0/accounts/{account_id}/workspaces/{workspace_id}/permissionassignments/principals/{principal_id} | Delete permissions assignment
[**workspace_assignmentget**](WorkspaceAssignmentApi.md#workspace_assignmentget) | **GET** /api/2.0/accounts/{account_id}/workspaces/{workspace_id}/permissionassignments/permissions | List workspace permissions
[**workspace_assignmentlist**](WorkspaceAssignmentApi.md#workspace_assignmentlist) | **GET** /api/2.0/accounts/{account_id}/workspaces/{workspace_id}/permissionassignments | Get permission assignments
[**workspace_assignmentupdate**](WorkspaceAssignmentApi.md#workspace_assignmentupdate) | **PUT** /api/2.0/accounts/{account_id}/workspaces/{workspace_id}/permissionassignments/principals/{principal_id} | Create or update permissions assignment



## workspace_assignmentdelete

> serde_json::Value workspace_assignmentdelete(account_id, workspace_id, principal_id)
Delete permissions assignment

Deletes the workspace permissions assignment in a given account and workspace for the specified principal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | The account ID. | [required] |
**workspace_id** | **i64** | The workspace ID. | [required] |
**principal_id** | **i64** | The ID of the user, service principal, or group. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_assignmentget

> crate::models::IamWorkspacePermissions workspace_assignmentget(account_id, workspace_id)
List workspace permissions

Get an array of workspace permissions for the specified account and workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | The account ID. | [required] |
**workspace_id** | [**serde_json::Value**](.md) | The workspace ID. | [required] |

### Return type

[**crate::models::IamWorkspacePermissions**](IamWorkspacePermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_assignmentlist

> crate::models::IamPermissionAssignments workspace_assignmentlist(account_id, workspace_id)
Get permission assignments

Get the permission assignments for the specified Databricks account and Databricks workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | The account ID. | [required] |
**workspace_id** | **i64** | The workspace ID for the account. | [required] |

### Return type

[**crate::models::IamPermissionAssignments**](IamPermissionAssignments.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspace_assignmentupdate

> serde_json::Value workspace_assignmentupdate(account_id, workspace_id, principal_id, iam_update_workspace_assignments)
Create or update permissions assignment

Creates or updates the workspace permissions assignment in a given account and workspace for the specified principal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | The account ID. | [required] |
**workspace_id** | **i64** | The workspace ID. | [required] |
**principal_id** | **i64** | The ID of the user, service principal, or group. | [required] |
**iam_update_workspace_assignments** | Option<[**IamUpdateWorkspaceAssignments**](IamUpdateWorkspaceAssignments.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

