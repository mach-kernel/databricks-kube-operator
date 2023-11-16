# \AccountMetastoreAssignmentsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_metastore_assignmentscreate**](AccountMetastoreAssignmentsApi.md#account_metastore_assignmentscreate) | **POST** /api/2.0/accounts/{account_id}/workspaces/{workspace_id}/metastores/{metastore_id} | Assigns a workspace to a metastore
[**account_metastore_assignmentsdelete**](AccountMetastoreAssignmentsApi.md#account_metastore_assignmentsdelete) | **DELETE** /api/2.0/accounts/{account_id}/workspaces/{workspace_id}/metastores/{metastore_id} | Delete a metastore assignment
[**account_metastore_assignmentsget**](AccountMetastoreAssignmentsApi.md#account_metastore_assignmentsget) | **GET** /api/2.0/accounts/{account_id}/workspaces/{workspace_id}/metastore | Gets the metastore assignment for a workspace
[**account_metastore_assignmentslist**](AccountMetastoreAssignmentsApi.md#account_metastore_assignmentslist) | **GET** /api/2.0/accounts/{account_id}/metastores/{metastore_id}/workspaces | Get all workspaces assigned to a metastore
[**account_metastore_assignmentsupdate**](AccountMetastoreAssignmentsApi.md#account_metastore_assignmentsupdate) | **PUT** /api/2.0/accounts/{account_id}/workspaces/{workspace_id}/metastores/{metastore_id} | Updates a metastore assignment to a workspaces



## account_metastore_assignmentscreate

> account_metastore_assignmentscreate(account_id, workspace_id, metastore_id, catalog_accounts_create_metastore_assignment)
Assigns a workspace to a metastore

Creates an assignment to a metastore for a workspace 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**workspace_id** | [**serde_json::Value**](.md) | Workspace ID. | [required] |
**metastore_id** | [**serde_json::Value**](.md) | Unity Catalog metastore ID | [required] |
**catalog_accounts_create_metastore_assignment** | [**CatalogAccountsCreateMetastoreAssignment**](CatalogAccountsCreateMetastoreAssignment.md) | The mapping from workspace to metastore. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_metastore_assignmentsdelete

> serde_json::Value account_metastore_assignmentsdelete(account_id, workspace_id, metastore_id)
Delete a metastore assignment

Deletes a metastore assignment to a workspace, leaving the workspace with no metastore. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**workspace_id** | [**serde_json::Value**](.md) | Workspace ID. | [required] |
**metastore_id** | [**serde_json::Value**](.md) | Unity Catalog metastore ID | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_metastore_assignmentsget

> crate::models::CatalogAccountsMetastoreAssignment account_metastore_assignmentsget(account_id, workspace_id)
Gets the metastore assignment for a workspace

Gets the metastore assignment, if any, for the workspace specified by ID. If the workspace is assigned a metastore, the mappig will be returned. If no metastore is assigned to the workspace, the assignment will not be found and a 404 returned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**workspace_id** | [**serde_json::Value**](.md) | Workspace ID. | [required] |

### Return type

[**crate::models::CatalogAccountsMetastoreAssignment**](CatalogAccountsMetastoreAssignment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_metastore_assignmentslist

> Vec<crate::models::CatalogMetastoreAssignment> account_metastore_assignmentslist(account_id, metastore_id)
Get all workspaces assigned to a metastore

Gets a list of all Databricks workspace IDs that have been assigned to given metastore. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**metastore_id** | [**serde_json::Value**](.md) | Unity Catalog metastore ID | [required] |

### Return type

[**Vec<crate::models::CatalogMetastoreAssignment>**](CatalogMetastoreAssignment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_metastore_assignmentsupdate

> serde_json::Value account_metastore_assignmentsupdate(account_id, workspace_id, metastore_id, catalog_accounts_update_metastore_assignment)
Updates a metastore assignment to a workspaces

Updates an assignment to a metastore for a workspace. Currently, only the default catalog may be updated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**workspace_id** | [**serde_json::Value**](.md) | Workspace ID. | [required] |
**metastore_id** | [**serde_json::Value**](.md) | Unity Catalog metastore ID | [required] |
**catalog_accounts_update_metastore_assignment** | [**CatalogAccountsUpdateMetastoreAssignment**](CatalogAccountsUpdateMetastoreAssignment.md) | The metastore assignment to update. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

