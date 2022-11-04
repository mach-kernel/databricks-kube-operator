# \DefaultApi

All URIs are relative to *https://<databricks-instance>/api/2.0*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_repo**](DefaultApi.md#create_repo) | **POST** /repos | Create a repo
[**delete_repo**](DefaultApi.md#delete_repo) | **DELETE** /repos/{repo_id} | Deletes the repo
[**get_repo**](DefaultApi.md#get_repo) | **GET** /repos/{repo_id} | Get a repo
[**get_repos**](DefaultApi.md#get_repos) | **GET** /repos | Get repos
[**update_repo**](DefaultApi.md#update_repo) | **PATCH** /repos/{repo_id} | Updates the repo to the given branch or tag



## create_repo

> crate::models::GetRepoResponse create_repo(create_repo_request)
Create a repo

Creates a repo in the workspace and links it to the remote Git repo specified. Note that repos created programmatically must be linked to a remote Git repo, unlike repos created in the browser.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_repo_request** | [**CreateRepoRequest**](CreateRepoRequest.md) | Details required to create and clone a repo object | [required] |

### Return type

[**crate::models::GetRepoResponse**](GetRepoResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_repo

> delete_repo(repo_id)
Deletes the repo

Deletes the specified repo

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_id** | **String** | The ID for the corresponding repo to access. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repo

> crate::models::GetRepoResponse get_repo(repo_id)
Get a repo

Returns the repo with the given repo ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_id** | **String** | The ID for the corresponding repo to access. | [required] |

### Return type

[**crate::models::GetRepoResponse**](GetRepoResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_repos

> crate::models::GetReposResponse get_repos(path_prefix, next_page_token)
Get repos

Returns repos that the calling user has Manage permissions on. Results are paginated with each page containing twenty repos.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path_prefix** | Option<**String**> | Filters repos that have paths starting with the given path prefix. |  |
**next_page_token** | Option<**String**> | Token used to get the next page of results. If not specified, returns the first page of results as well as a next page token if there are more results. |  |

### Return type

[**crate::models::GetReposResponse**](GetReposResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_repo

> crate::models::GetRepoResponse update_repo(repo_id, update_repo_request)
Updates the repo to the given branch or tag

Updates the repo to a different branch or tag, or updates the repo to the latest commit on the same branch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**repo_id** | **String** | The ID for the corresponding repo to access. | [required] |
**update_repo_request** | [**UpdateRepoRequest**](UpdateRepoRequest.md) | Details required to update the repo | [required] |

### Return type

[**crate::models::GetRepoResponse**](GetRepoResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

