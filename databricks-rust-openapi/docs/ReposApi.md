# \ReposApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reposcreate**](ReposApi.md#reposcreate) | **POST** /api/2.0/repos | Create a repo
[**reposdelete**](ReposApi.md#reposdelete) | **DELETE** /api/2.0/repos/{repo_id} | Delete a repo
[**reposget**](ReposApi.md#reposget) | **GET** /api/2.0/repos/{repo_id} | Get a repo
[**reposget_repo_permission_levels**](ReposApi.md#reposget_repo_permission_levels) | **GET** /api/2.0/permissions/repos/{repo_id}/permissionLevels | Get repo permission levels
[**reposget_repo_permissions**](ReposApi.md#reposget_repo_permissions) | **GET** /api/2.0/permissions/repos/{repo_id} | Get repo permissions
[**reposlist**](ReposApi.md#reposlist) | **GET** /api/2.0/repos | Get repos
[**reposset_repo_permissions**](ReposApi.md#reposset_repo_permissions) | **PUT** /api/2.0/permissions/repos/{repo_id} | Set repo permissions
[**reposupdate**](ReposApi.md#reposupdate) | **PATCH** /api/2.0/repos/{repo_id} | Update a repo
[**reposupdate_repo_permissions**](ReposApi.md#reposupdate_repo_permissions) | **PATCH** /api/2.0/permissions/repos/{repo_id} | Update repo permissions



## reposcreate

> crate::models::WorkspaceRepoInfo reposcreate(workspace_create_repo)
Create a repo

Creates a repo in the workspace and links it to the remote Git repo specified.  Note that repos created programmatically must be linked to a remote Git repo, unlike repos created in the browser. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_create_repo** | [**WorkspaceCreateRepo**](WorkspaceCreateRepo.md) |  | [required] |

### Return type

[**crate::models::WorkspaceRepoInfo**](WorkspaceRepoInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reposdelete

> reposdelete(repo_id)
Delete a repo

Deletes the specified repo.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_id** | **i64** | The ID for the corresponding repo to access. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reposget

> crate::models::WorkspaceRepoInfo reposget(repo_id)
Get a repo

Returns the repo with the given repo ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_id** | **i64** | The ID for the corresponding repo to access. | [required] |

### Return type

[**crate::models::WorkspaceRepoInfo**](WorkspaceRepoInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reposget_repo_permission_levels

> crate::models::WorkspaceGetRepoPermissionLevelsResponse reposget_repo_permission_levels(repo_id)
Get repo permission levels

Gets the permission levels that a user can have on an object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_id** | **String** | The repo for which to get or manage permissions. | [required] |

### Return type

[**crate::models::WorkspaceGetRepoPermissionLevelsResponse**](WorkspaceGetRepoPermissionLevelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reposget_repo_permissions

> crate::models::WorkspaceRepoPermissions reposget_repo_permissions(repo_id)
Get repo permissions

Gets the permissions of a repo. Repos can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_id** | [**serde_json::Value**](.md) | The repo for which to get or manage permissions. | [required] |

### Return type

[**crate::models::WorkspaceRepoPermissions**](WorkspaceRepoPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reposlist

> crate::models::WorkspaceListReposResponse reposlist(path_prefix, next_page_token)
Get repos

Returns repos that the calling user has Manage permissions on. Results are paginated with each page containing twenty repos.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path_prefix** | Option<**String**> | Filters repos that have paths starting with the given path prefix. |  |
**next_page_token** | Option<**String**> | Token used to get the next page of results. If not specified, returns the first page of results as well as a next page token if there are more results. |  |

### Return type

[**crate::models::WorkspaceListReposResponse**](WorkspaceListReposResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reposset_repo_permissions

> crate::models::WorkspaceRepoPermissions reposset_repo_permissions(repo_id, workspace_repo_permissions_request)
Set repo permissions

Sets permissions on a repo. Repos can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_id** | [**serde_json::Value**](.md) | The repo for which to get or manage permissions. | [required] |
**workspace_repo_permissions_request** | Option<[**WorkspaceRepoPermissionsRequest**](WorkspaceRepoPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::WorkspaceRepoPermissions**](WorkspaceRepoPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reposupdate

> serde_json::Value reposupdate(repo_id, workspace_update_repo)
Update a repo

Updates the repo to a different branch or tag, or updates the repo to the latest commit on the same branch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_id** | **i64** | The ID for the corresponding repo to access. | [required] |
**workspace_update_repo** | [**WorkspaceUpdateRepo**](WorkspaceUpdateRepo.md) | Details required to update the repo | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reposupdate_repo_permissions

> crate::models::WorkspaceRepoPermissions reposupdate_repo_permissions(repo_id, workspace_repo_permissions_request)
Update repo permissions

Updates the permissions on a repo. Repos can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_id** | [**serde_json::Value**](.md) | The repo for which to get or manage permissions. | [required] |
**workspace_repo_permissions_request** | Option<[**WorkspaceRepoPermissionsRequest**](WorkspaceRepoPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::WorkspaceRepoPermissions**](WorkspaceRepoPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

