# \ClustersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clusterschange_owner**](ClustersApi.md#clusterschange_owner) | **POST** /api/2.0/clusters/change-owner | Change cluster owner
[**clusterscreate**](ClustersApi.md#clusterscreate) | **POST** /api/2.0/clusters/create | Create new cluster
[**clustersdelete**](ClustersApi.md#clustersdelete) | **POST** /api/2.0/clusters/delete | Terminate cluster
[**clustersedit**](ClustersApi.md#clustersedit) | **POST** /api/2.0/clusters/edit | Update cluster configuration
[**clustersevents**](ClustersApi.md#clustersevents) | **POST** /api/2.0/clusters/events | List cluster activity events
[**clustersget**](ClustersApi.md#clustersget) | **GET** /api/2.0/clusters/get | Get cluster info
[**clustersget_cluster_permission_levels**](ClustersApi.md#clustersget_cluster_permission_levels) | **GET** /api/2.0/permissions/clusters/{cluster_id}/permissionLevels | Get cluster permission levels
[**clustersget_cluster_permissions**](ClustersApi.md#clustersget_cluster_permissions) | **GET** /api/2.0/permissions/clusters/{cluster_id} | Get cluster permissions
[**clusterslist**](ClustersApi.md#clusterslist) | **GET** /api/2.0/clusters/list | List all clusters
[**clusterslist_node_types**](ClustersApi.md#clusterslist_node_types) | **GET** /api/2.0/clusters/list-node-types | List node types
[**clusterslist_zones**](ClustersApi.md#clusterslist_zones) | **GET** /api/2.0/clusters/list-zones | List availability zones
[**clusterspermanent_delete**](ClustersApi.md#clusterspermanent_delete) | **POST** /api/2.0/clusters/permanent-delete | Permanently delete cluster
[**clusterspin**](ClustersApi.md#clusterspin) | **POST** /api/2.0/clusters/pin | Pin cluster
[**clustersresize**](ClustersApi.md#clustersresize) | **POST** /api/2.0/clusters/resize | Resize cluster
[**clustersrestart**](ClustersApi.md#clustersrestart) | **POST** /api/2.0/clusters/restart | Restart cluster
[**clustersset_cluster_permissions**](ClustersApi.md#clustersset_cluster_permissions) | **PUT** /api/2.0/permissions/clusters/{cluster_id} | Set cluster permissions
[**clustersspark_versions**](ClustersApi.md#clustersspark_versions) | **GET** /api/2.0/clusters/spark-versions | List available Spark versions
[**clustersstart**](ClustersApi.md#clustersstart) | **POST** /api/2.0/clusters/start | Start terminated cluster
[**clustersunpin**](ClustersApi.md#clustersunpin) | **POST** /api/2.0/clusters/unpin | Unpin cluster
[**clustersupdate_cluster_permissions**](ClustersApi.md#clustersupdate_cluster_permissions) | **PATCH** /api/2.0/permissions/clusters/{cluster_id} | Update cluster permissions



## clusterschange_owner

> serde_json::Value clusterschange_owner(compute_change_cluster_owner)
Change cluster owner

Change the owner of the cluster. You must be an admin to perform this operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_change_cluster_owner** | Option<[**ComputeChangeClusterOwner**](ComputeChangeClusterOwner.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusterscreate

> crate::models::ComputeCreateClusterResponse clusterscreate(compute_create_cluster)
Create new cluster

Creates a new Spark cluster. This method will acquire new instances from the cloud provider if necessary. Note: Databricks may not be able to acquire some of the requested nodes, due to cloud provider limitations (account limits, spot price, etc.) or transient network issues.  If Databricks acquires at least 85% of the requested on-demand nodes, cluster creation will succeed. Otherwise the cluster will terminate with an informative error message. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_create_cluster** | Option<[**ComputeCreateCluster**](ComputeCreateCluster.md)> |  |  |

### Return type

[**crate::models::ComputeCreateClusterResponse**](ComputeCreateClusterResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clustersdelete

> serde_json::Value clustersdelete(compute_delete_cluster)
Terminate cluster

Terminates the Spark cluster with the specified ID. The cluster is removed asynchronously. Once the termination has completed, the cluster will be in a `TERMINATED` state. If the cluster is already in a `TERMINATING` or `TERMINATED` state, nothing will happen. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_delete_cluster** | Option<[**ComputeDeleteCluster**](ComputeDeleteCluster.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clustersedit

> serde_json::Value clustersedit(compute_edit_cluster)
Update cluster configuration

Updates the configuration of a cluster to match the provided attributes and size. A cluster can be updated if it is in a `RUNNING` or `TERMINATED` state.  If a cluster is updated while in a `RUNNING` state, it will be restarted so that the new attributes can take effect.  If a cluster is updated while in a `TERMINATED` state, it will remain `TERMINATED`. The next time it is started using the `clusters/start` API, the new attributes will take effect. Any attempt to update a cluster in any other state will be rejected with an `INVALID_STATE` error code.  Clusters created by the Databricks Jobs service cannot be edited. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_edit_cluster** | Option<[**ComputeEditCluster**](ComputeEditCluster.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clustersevents

> crate::models::ComputeGetEventsResponse clustersevents(compute_get_events)
List cluster activity events

Retrieves a list of events about the activity of a cluster. This API is paginated. If there are more events to read, the response includes all the nparameters necessary to request the next page of events. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_get_events** | Option<[**ComputeGetEvents**](ComputeGetEvents.md)> |  |  |

### Return type

[**crate::models::ComputeGetEventsResponse**](ComputeGetEventsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clustersget

> crate::models::ComputeClusterDetails clustersget(cluster_id)
Get cluster info

Retrieves the information for a cluster given its identifier. Clusters can be described while they are running, or up to 60 days after they are terminated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **String** | The cluster about which to retrieve information. | [required] |

### Return type

[**crate::models::ComputeClusterDetails**](ComputeClusterDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clustersget_cluster_permission_levels

> crate::models::ComputeGetClusterPermissionLevelsResponse clustersget_cluster_permission_levels(cluster_id)
Get cluster permission levels

Gets the permission levels that a user can have on an object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | **String** | The cluster for which to get or manage permissions. | [required] |

### Return type

[**crate::models::ComputeGetClusterPermissionLevelsResponse**](ComputeGetClusterPermissionLevelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clustersget_cluster_permissions

> crate::models::ComputeClusterPermissions clustersget_cluster_permissions(cluster_id)
Get cluster permissions

Gets the permissions of a cluster. Clusters can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | [**serde_json::Value**](.md) | The cluster for which to get or manage permissions. | [required] |

### Return type

[**crate::models::ComputeClusterPermissions**](ComputeClusterPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusterslist

> crate::models::ComputeListClustersResponse clusterslist(can_use_client)
List all clusters

Return information about all pinned clusters, active clusters, up to 200 of the most recently terminated all-purpose clusters in the past 30 days, and up to 30 of the most recently terminated job clusters in the past 30 days.  For example, if there is 1 pinned cluster, 4 active clusters, 45 terminated all-purpose clusters in the past 30 days, and 50 terminated job clusters in the past 30 days, then this API returns the 1 pinned cluster, 4 active clusters, all 45 terminated all-purpose clusters, and the 30 most recently terminated job clusters. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**can_use_client** | Option<**String**> | Filter clusters based on what type of client it can be used for. Could be either NOTEBOOKS or JOBS. No input for this field will get all clusters in the workspace without filtering on its supported client |  |

### Return type

[**crate::models::ComputeListClustersResponse**](ComputeListClustersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusterslist_node_types

> crate::models::ComputeListNodeTypesResponse clusterslist_node_types()
List node types

Returns a list of supported Spark node types. These node types can be used to launch a cluster.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ComputeListNodeTypesResponse**](ComputeListNodeTypesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusterslist_zones

> crate::models::ComputeListAvailableZonesResponse clusterslist_zones()
List availability zones

Returns a list of availability zones where clusters can be created in (For example, us-west-2a). These zones can be used to launch a cluster. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ComputeListAvailableZonesResponse**](ComputeListAvailableZonesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusterspermanent_delete

> serde_json::Value clusterspermanent_delete(compute_permanent_delete_cluster)
Permanently delete cluster

Permanently deletes a Spark cluster. This cluster is terminated and resources are asynchronously removed.  In addition, users will no longer see permanently deleted clusters in the cluster list, and API users can no longer perform any action on permanently deleted clusters. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_permanent_delete_cluster** | Option<[**ComputePermanentDeleteCluster**](ComputePermanentDeleteCluster.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clusterspin

> serde_json::Value clusterspin(compute_pin_cluster)
Pin cluster

Pinning a cluster ensures that the cluster will always be returned by the ListClusters API. Pinning a cluster that is already pinned will have no effect. This API can only be called by workspace admins. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_pin_cluster** | Option<[**ComputePinCluster**](ComputePinCluster.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clustersresize

> serde_json::Value clustersresize(compute_resize_cluster)
Resize cluster

Resizes a cluster to have a desired number of workers. This will fail unless the cluster is in a `RUNNING` state. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_resize_cluster** | Option<[**ComputeResizeCluster**](ComputeResizeCluster.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clustersrestart

> serde_json::Value clustersrestart(compute_restart_cluster)
Restart cluster

Restarts a Spark cluster with the supplied ID. If the cluster is not currently in a `RUNNING` state, nothing will happen. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_restart_cluster** | Option<[**ComputeRestartCluster**](ComputeRestartCluster.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clustersset_cluster_permissions

> crate::models::ComputeClusterPermissions clustersset_cluster_permissions(cluster_id, compute_cluster_permissions_request)
Set cluster permissions

Sets permissions on a cluster. Clusters can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | [**serde_json::Value**](.md) | The cluster for which to get or manage permissions. | [required] |
**compute_cluster_permissions_request** | Option<[**ComputeClusterPermissionsRequest**](ComputeClusterPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::ComputeClusterPermissions**](ComputeClusterPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clustersspark_versions

> crate::models::ComputeGetSparkVersionsResponse clustersspark_versions()
List available Spark versions

Returns the list of available Spark versions. These versions can be used to launch a cluster.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ComputeGetSparkVersionsResponse**](ComputeGetSparkVersionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clustersstart

> serde_json::Value clustersstart(compute_start_cluster)
Start terminated cluster

Starts a terminated Spark cluster with the supplied ID. This works similar to `createCluster` except:  * The previous cluster id and attributes are preserved. * The cluster starts with the last specified cluster size. * If the previous cluster was an autoscaling cluster, the current cluster starts with the minimum number of nodes. * If the cluster is not currently in a `TERMINATED` state, nothing will happen. * Clusters launched to run a job cannot be started. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_start_cluster** | Option<[**ComputeStartCluster**](ComputeStartCluster.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clustersunpin

> serde_json::Value clustersunpin(compute_unpin_cluster)
Unpin cluster

Unpinning a cluster will allow the cluster to eventually be removed from the ListClusters API. Unpinning a cluster that is not pinned will have no effect. This API can only be called by workspace admins. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_unpin_cluster** | Option<[**ComputeUnpinCluster**](ComputeUnpinCluster.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clustersupdate_cluster_permissions

> crate::models::ComputeClusterPermissions clustersupdate_cluster_permissions(cluster_id, compute_cluster_permissions_request)
Update cluster permissions

Updates the permissions on a cluster. Clusters can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_id** | [**serde_json::Value**](.md) | The cluster for which to get or manage permissions. | [required] |
**compute_cluster_permissions_request** | Option<[**ComputeClusterPermissionsRequest**](ComputeClusterPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::ComputeClusterPermissions**](ComputeClusterPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

