# \InstanceProfilesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**instance_profilesadd**](InstanceProfilesApi.md#instance_profilesadd) | **POST** /api/2.0/instance-profiles/add | Register an instance profile
[**instance_profilesedit**](InstanceProfilesApi.md#instance_profilesedit) | **POST** /api/2.0/instance-profiles/edit | Edit an instance profile
[**instance_profileslist**](InstanceProfilesApi.md#instance_profileslist) | **GET** /api/2.0/instance-profiles/list | List available instance profiles
[**instance_profilesremove**](InstanceProfilesApi.md#instance_profilesremove) | **POST** /api/2.0/instance-profiles/remove | Remove the instance profile



## instance_profilesadd

> serde_json::Value instance_profilesadd(compute_add_instance_profile)
Register an instance profile

In the UI, you can select the instance profile when launching clusters. This API is only available to admin users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_add_instance_profile** | Option<[**ComputeAddInstanceProfile**](ComputeAddInstanceProfile.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_profilesedit

> serde_json::Value instance_profilesedit(compute_instance_profile)
Edit an instance profile

The only supported field to change is the optional IAM role ARN associated with the instance profile. It is required to specify the IAM role ARN if both of the following are true:   * Your role name and instance profile name do not match. The name is the part    after the last slash in each ARN.  * You want to use the instance profile with [Databricks SQL Serverless](https://Docsdatabricks.com/sql/admin/serverless.html).  To understand where these fields are in the AWS console, see [Enable serverless SQL warehouses](https://Docsdatabricks.com/sql/admin/serverless.html).  This API is only available to admin users. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_instance_profile** | Option<[**ComputeInstanceProfile**](ComputeInstanceProfile.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_profileslist

> crate::models::ComputeListInstanceProfilesResponse instance_profileslist()
List available instance profiles

List the instance profiles that the calling user can use to launch a cluster.  This API is available to all users. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ComputeListInstanceProfilesResponse**](ComputeListInstanceProfilesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## instance_profilesremove

> serde_json::Value instance_profilesremove(compute_remove_instance_profile)
Remove the instance profile

Remove the instance profile with the provided ARN. Existing clusters with this instance profile will continue to function.  This API is only accessible to admin users. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**compute_remove_instance_profile** | Option<[**ComputeRemoveInstanceProfile**](ComputeRemoveInstanceProfile.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

