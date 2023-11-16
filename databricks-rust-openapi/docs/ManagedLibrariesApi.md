# \ManagedLibrariesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**librariesall_cluster_library_statuses**](ManagedLibrariesApi.md#librariesall_cluster_library_statuses) | **GET** /api/2.0/libraries/all-cluster-statuses | Get all statuses
[**librariescluster_status**](ManagedLibrariesApi.md#librariescluster_status) | **GET** /api/2.0/libraries/cluster-status | Get status
[**librariesinstall**](ManagedLibrariesApi.md#librariesinstall) | **POST** /api/2.0/libraries/install | Add a library
[**librariesuninstall**](ManagedLibrariesApi.md#librariesuninstall) | **POST** /api/2.0/libraries/uninstall | Uninstall libraries



## librariesall_cluster_library_statuses

> crate::models::ComputeListAllClusterLibraryStatusesResponse librariesall_cluster_library_statuses()
Get all statuses

Get the status of all libraries on all clusters. A status will be available for all libraries installed on this cluster  via the API or the libraries UI as well as libraries set to be installed on all clusters via the libraries UI. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ComputeListAllClusterLibraryStatusesResponse**](ComputeListAllClusterLibraryStatusesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## librariescluster_status

> crate::models::ComputeClusterLibraryStatuses librariescluster_status(cluster_id)
Get status

Get the status of libraries on a cluster. A status will be available for all libraries installed on this cluster via the API  or the libraries UI as well as libraries set to be installed on all clusters via the libraries UI.  The order of returned libraries will be as follows.  1. Libraries set to be installed on this cluster will be returned first.    Within this group, the final order will be order in which the libraries were added to the cluster.  2. Libraries set to be installed on all clusters are returned next.    Within this group there is no order guarantee.  3. Libraries that were previously requested on this cluster or on all clusters, but now marked for removal.    Within this group there is no order guarantee. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **String** | Unique identifier of the cluster whose status should be retrieved. | [required] |

### Return type

[**crate::models::ComputeClusterLibraryStatuses**](ComputeClusterLibraryStatuses.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## librariesinstall

> serde_json::Value librariesinstall(compute_install_libraries)
Add a library

Add libraries to be installed on a cluster.  The installation is asynchronous; it happens in the background after the completion of this request.   **Note**: The actual set of libraries to be installed on a cluster is the union of the libraries specified via this method and  the libraries set to be installed on all clusters via the libraries UI. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_install_libraries** | Option<[**ComputeInstallLibraries**](ComputeInstallLibraries.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## librariesuninstall

> serde_json::Value librariesuninstall(compute_uninstall_libraries)
Uninstall libraries

Set libraries to be uninstalled on a cluster. The libraries won't be uninstalled until the cluster is restarted.  Uninstalling libraries that are not installed on the cluster will have no impact but is not an error. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_uninstall_libraries** | Option<[**ComputeUninstallLibraries**](ComputeUninstallLibraries.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

