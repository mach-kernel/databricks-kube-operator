# \AccountAccessControlApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_access_controlget_assignable_roles_for_resource**](AccountAccessControlApi.md#account_access_controlget_assignable_roles_for_resource) | **GET** /api/2.0/preview/accounts/{account_id}/access-control/assignable-roles | Get assignable roles for a resource
[**account_access_controlget_rule_set**](AccountAccessControlApi.md#account_access_controlget_rule_set) | **GET** /api/2.0/preview/accounts/{account_id}/access-control/rule-sets | Get a rule set
[**account_access_controlupdate_rule_set**](AccountAccessControlApi.md#account_access_controlupdate_rule_set) | **PUT** /api/2.0/preview/accounts/{account_id}/access-control/rule-sets | Update a rule set



## account_access_controlget_assignable_roles_for_resource

> crate::models::IamGetAssignableRolesForResourceResponse account_access_controlget_assignable_roles_for_resource(resource)
Get assignable roles for a resource

Gets all the roles that can be granted on an account level resource. A role is grantable if the rule set on the resource can contain an access rule of the role. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource** | [**serde_json::Value**](.md) | The resource name for which assignable roles will be listed. | [required] |

### Return type

[**crate::models::IamGetAssignableRolesForResourceResponse**](IamGetAssignableRolesForResourceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_access_controlget_rule_set

> crate::models::IamRuleSetResponse account_access_controlget_rule_set(name, etag)
Get a rule set

Get a rule set by its name. A rule set is always attached to a resource and contains a list of access rules on the said resource. Currently only a default rule set for each resource is supported. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**serde_json::Value**](.md) | The ruleset name associated with the request. | [required] |
**etag** | [**serde_json::Value**](.md) | Etag used for versioning. The response is at least as fresh as the eTag provided. Etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a rule set from overwriting each other. It is strongly suggested that systems make use of the etag in the read -> modify -> write pattern to perform rule set updates in order to avoid race conditions that is get an etag from a GET rule set request, and pass it with the PUT update request to identify the rule set version you are updating.  | [required] |

### Return type

[**crate::models::IamRuleSetResponse**](IamRuleSetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_access_controlupdate_rule_set

> crate::models::IamRuleSetResponse account_access_controlupdate_rule_set(account_id, iam_update_rule_set_request)
Update a rule set

Replace the rules of a rule set. First, use  get to read the current version of the rule set before modifying it. This pattern helps prevent conflicts between concurrent updates. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. | [required] |
**iam_update_rule_set_request** | [**IamUpdateRuleSetRequest**](IamUpdateRuleSetRequest.md) | The rule set to update to. | [required] |

### Return type

[**crate::models::IamRuleSetResponse**](IamRuleSetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

