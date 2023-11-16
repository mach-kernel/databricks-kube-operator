# \PolicyFamiliesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**policy_familiesget**](PolicyFamiliesApi.md#policy_familiesget) | **GET** /api/2.0/policy-families/{policy_family_id} | Get policy family information
[**policy_familieslist**](PolicyFamiliesApi.md#policy_familieslist) | **GET** /api/2.0/policy-families | List policy families



## policy_familiesget

> crate::models::ComputePolicyFamily policy_familiesget(policy_family_id)
Get policy family information

Retrieve the information for an policy family based on its identifier. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_family_id** | **String** |  | [required] |

### Return type

[**crate::models::ComputePolicyFamily**](ComputePolicyFamily.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## policy_familieslist

> crate::models::ComputeListPolicyFamiliesResponse policy_familieslist(max_results, page_token)
List policy families

Retrieve a list of policy families. This API is paginated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max_results** | Option<**i64**> | The max number of policy families to return. |  |
**page_token** | Option<**String**> | A token that can be used to get the next page of results. |  |

### Return type

[**crate::models::ComputeListPolicyFamiliesResponse**](ComputeListPolicyFamiliesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

