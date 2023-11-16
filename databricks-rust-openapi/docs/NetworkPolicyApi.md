# \NetworkPolicyApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_network_policydelete_account_network_policy**](NetworkPolicyApi.md#account_network_policydelete_account_network_policy) | **DELETE** /api/2.0/accounts/{account_id}/settings/types/network_policy/names/default | Delete Account Network Policy
[**account_network_policyread_account_network_policy**](NetworkPolicyApi.md#account_network_policyread_account_network_policy) | **GET** /api/2.0/accounts/{account_id}/settings/types/network_policy/names/default | Get Account Network Policy
[**account_network_policyupdate_account_network_policy**](NetworkPolicyApi.md#account_network_policyupdate_account_network_policy) | **PATCH** /api/2.0/accounts/{account_id}/settings/types/network_policy/names/default | Update Account Network Policy



## account_network_policydelete_account_network_policy

> crate::models::AccountSettingsdeletePersonalComputeSetting200Response account_network_policydelete_account_network_policy(etag)
Delete Account Network Policy

Reverts back all the account network policies back to default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**etag** | [**serde_json::Value**](.md) | etag used for versioning. The response is at least as fresh as the eTag provided. This is used for optimistic concurrency control as a way to help prevent simultaneous writes of a setting overwriting each other. It is strongly suggested that systems make use of the etag in the read -> delete pattern to perform setting deletions in order to avoid race conditions. That is, get an etag from a GET request, and pass it with the DELETE request to identify the rule set version you are deleting.  | [required] |

### Return type

[**crate::models::AccountSettingsdeletePersonalComputeSetting200Response**](AccountSettingsdeletePersonalComputeSetting_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_network_policyread_account_network_policy

> crate::models::SettingsAccountNetworkPolicyMessage account_network_policyread_account_network_policy(etag)
Get Account Network Policy

Gets the value of Account level Network Policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**etag** | [**serde_json::Value**](.md) | etag used for versioning. The response is at least as fresh as the eTag provided. This is used for optimistic concurrency control as a way to help prevent simultaneous writes of a setting overwriting each other. It is strongly suggested that systems make use of the etag in the read -> delete pattern to perform setting deletions in order to avoid race conditions. That is, get an etag from a GET request, and pass it with the DELETE request to identify the rule set version you are deleting.  | [required] |

### Return type

[**crate::models::SettingsAccountNetworkPolicyMessage**](SettingsAccountNetworkPolicyMessage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_network_policyupdate_account_network_policy

> crate::models::SettingsAccountNetworkPolicyMessage account_network_policyupdate_account_network_policy(account_id, account_network_policyupdate_account_network_policy_request)
Update Account Network Policy

Updates the policy content of Account level Network Policy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of the account being managed. | [required] |
**account_network_policyupdate_account_network_policy_request** | [**AccountNetworkPolicyupdateAccountNetworkPolicyRequest**](AccountNetworkPolicyupdateAccountNetworkPolicyRequest.md) | Policy content of Account level Network Policy. | [required] |

### Return type

[**crate::models::SettingsAccountNetworkPolicyMessage**](SettingsAccountNetworkPolicyMessage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

