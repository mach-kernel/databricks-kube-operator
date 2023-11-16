# \ClusterPoliciesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cluster_policiescreate**](ClusterPoliciesApi.md#cluster_policiescreate) | **POST** /api/2.0/policies/clusters/create | Create a new policy
[**cluster_policiesdelete**](ClusterPoliciesApi.md#cluster_policiesdelete) | **POST** /api/2.0/policies/clusters/delete | Delete a cluster policy
[**cluster_policiesedit**](ClusterPoliciesApi.md#cluster_policiesedit) | **POST** /api/2.0/policies/clusters/edit | Update a cluster policy
[**cluster_policiesget**](ClusterPoliciesApi.md#cluster_policiesget) | **GET** /api/2.0/policies/clusters/get | Get entity
[**cluster_policiesget_cluster_policy_permission_levels**](ClusterPoliciesApi.md#cluster_policiesget_cluster_policy_permission_levels) | **GET** /api/2.0/permissions/cluster-policies/{cluster_policy_id}/permissionLevels | Get cluster policy permission levels
[**cluster_policiesget_cluster_policy_permissions**](ClusterPoliciesApi.md#cluster_policiesget_cluster_policy_permissions) | **GET** /api/2.0/permissions/cluster-policies/{cluster_policy_id} | Get cluster policy permissions
[**cluster_policieslist**](ClusterPoliciesApi.md#cluster_policieslist) | **GET** /api/2.0/policies/clusters/list | Get a cluster policy
[**cluster_policiesset_cluster_policy_permissions**](ClusterPoliciesApi.md#cluster_policiesset_cluster_policy_permissions) | **PUT** /api/2.0/permissions/cluster-policies/{cluster_policy_id} | Set cluster policy permissions
[**cluster_policiesupdate_cluster_policy_permissions**](ClusterPoliciesApi.md#cluster_policiesupdate_cluster_policy_permissions) | **PATCH** /api/2.0/permissions/cluster-policies/{cluster_policy_id} | Update cluster policy permissions



## cluster_policiescreate

> crate::models::ComputeCreatePolicyResponse cluster_policiescreate(compute_create_policy)
Create a new policy

Creates a new policy with prescribed settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_create_policy** | Option<[**ComputeCreatePolicy**](ComputeCreatePolicy.md)> |  |  |

### Return type

[**crate::models::ComputeCreatePolicyResponse**](ComputeCreatePolicyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_policiesdelete

> serde_json::Value cluster_policiesdelete(compute_delete_policy)
Delete a cluster policy

Delete a policy for a cluster. Clusters governed by this policy can still run, but cannot be edited.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_delete_policy** | Option<[**ComputeDeletePolicy**](ComputeDeletePolicy.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_policiesedit

> serde_json::Value cluster_policiesedit(compute_edit_policy)
Update a cluster policy

Update an existing policy for cluster. This operation may make some clusters governed by the previous policy invalid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_edit_policy** | Option<[**ComputeEditPolicy**](ComputeEditPolicy.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_policiesget

> crate::models::ComputePolicy cluster_policiesget(policy_id)
Get entity

Get a cluster policy entity. Creation and editing is available to admins only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **String** | Canonical unique identifier for the cluster policy. | [required] |

### Return type

[**crate::models::ComputePolicy**](ComputePolicy.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_policiesget_cluster_policy_permission_levels

> crate::models::ComputeGetClusterPolicyPermissionLevelsResponse cluster_policiesget_cluster_policy_permission_levels(cluster_policy_id)
Get cluster policy permission levels

Gets the permission levels that a user can have on an object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_policy_id** | **String** | The cluster policy for which to get or manage permissions. | [required] |

### Return type

[**crate::models::ComputeGetClusterPolicyPermissionLevelsResponse**](ComputeGetClusterPolicyPermissionLevelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_policiesget_cluster_policy_permissions

> crate::models::ComputeClusterPolicyPermissions cluster_policiesget_cluster_policy_permissions(cluster_policy_id)
Get cluster policy permissions

Gets the permissions of a cluster policy. Cluster policies can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_policy_id** | [**serde_json::Value**](.md) | The cluster policy for which to get or manage permissions. | [required] |

### Return type

[**crate::models::ComputeClusterPolicyPermissions**](ComputeClusterPolicyPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_policieslist

> crate::models::ComputeListPoliciesResponse cluster_policieslist(sort_order, sort_column)
Get a cluster policy

Returns a list of policies accessible by the requesting user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort_order** | Option<**String**> | The order in which the policies get listed. * `DESC` - Sort result list in descending order. * `ASC` - Sort result list in ascending order.  |  |[default to DESC]
**sort_column** | Option<**String**> | The cluster policy attribute to sort by. * `POLICY_CREATION_TIME` - Sort result list by policy creation time. * `POLICY_NAME` - Sort result list by policy name.  |  |

### Return type

[**crate::models::ComputeListPoliciesResponse**](ComputeListPoliciesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_policiesset_cluster_policy_permissions

> crate::models::ComputeClusterPolicyPermissions cluster_policiesset_cluster_policy_permissions(cluster_policy_id, compute_cluster_policy_permissions_request)
Set cluster policy permissions

Sets permissions on a cluster policy. Cluster policies can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_policy_id** | [**serde_json::Value**](.md) | The cluster policy for which to get or manage permissions. | [required] |
**compute_cluster_policy_permissions_request** | Option<[**ComputeClusterPolicyPermissionsRequest**](ComputeClusterPolicyPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::ComputeClusterPolicyPermissions**](ComputeClusterPolicyPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cluster_policiesupdate_cluster_policy_permissions

> crate::models::ComputeClusterPolicyPermissions cluster_policiesupdate_cluster_policy_permissions(cluster_policy_id, compute_cluster_policy_permissions_request)
Update cluster policy permissions

Updates the permissions on a cluster policy. Cluster policies can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cluster_policy_id** | [**serde_json::Value**](.md) | The cluster policy for which to get or manage permissions. | [required] |
**compute_cluster_policy_permissions_request** | Option<[**ComputeClusterPolicyPermissionsRequest**](ComputeClusterPolicyPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::ComputeClusterPolicyPermissions**](ComputeClusterPolicyPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

