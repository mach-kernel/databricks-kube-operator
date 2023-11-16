# \AccountGroupsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_groupscreate**](AccountGroupsApi.md#account_groupscreate) | **POST** /api/2.0/accounts/{account_id}/scim/v2/Groups | Create a new group
[**account_groupsdelete**](AccountGroupsApi.md#account_groupsdelete) | **DELETE** /api/2.0/accounts/{account_id}/scim/v2/Groups/{id} | Delete a group
[**account_groupsget**](AccountGroupsApi.md#account_groupsget) | **GET** /api/2.0/accounts/{account_id}/scim/v2/Groups/{id} | Get group details
[**account_groupslist**](AccountGroupsApi.md#account_groupslist) | **GET** /api/2.0/accounts/{account_id}/scim/v2/Groups | List group details
[**account_groupspatch**](AccountGroupsApi.md#account_groupspatch) | **PATCH** /api/2.0/accounts/{account_id}/scim/v2/Groups/{id} | Update group details
[**account_groupsupdate**](AccountGroupsApi.md#account_groupsupdate) | **PUT** /api/2.0/accounts/{account_id}/scim/v2/Groups/{id} | Replace a group



## account_groupscreate

> crate::models::IamGroup account_groupscreate(account_id, iam_group)
Create a new group

Creates a group in the Databricks account with a unique name, using the supplied group details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID | [required] |
**iam_group** | [**IamGroup**](IamGroup.md) | Properties of the new group. | [required] |

### Return type

[**crate::models::IamGroup**](IamGroup.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_groupsdelete

> serde_json::Value account_groupsdelete(account_id, id)
Delete a group

Deletes a group from the Databricks account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Databricks account ID | [required] |
**id** | [**serde_json::Value**](.md) | Unique ID for a group in the Databricks account. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_groupsget

> crate::models::IamGroup account_groupsget(account_id, id)
Get group details

Gets the information for a specific group in the Databricks account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Databricks account ID | [required] |
**id** | [**serde_json::Value**](.md) | Unique ID for a group in the Databricks account. | [required] |

### Return type

[**crate::models::IamGroup**](IamGroup.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_groupslist

> crate::models::IamListGroupsResponse account_groupslist(account_id, filter, attributes, excluded_attributes, start_index, count, sort_by, sort_order)
List group details

Gets all details of the groups associated with the Databricks account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID | [required] |
**filter** | Option<**String**> | Query by which the results have to be filtered. Supported operators are equals(`eq`), contains(`co`), starts with(`sw`) and not equals(`ne`). Additionally, simple expressions can be formed using logical operators - `and` and `or`. The [SCIM RFC](https://Toolsietf.org/html/rfc7644#section-3.4.2.2) has more details but we currently only support simple expressions. |  |
**attributes** | Option<**String**> | Comma-separated list of attributes to return in response. |  |
**excluded_attributes** | Option<**String**> | Comma-separated list of attributes to exclude in response. |  |
**start_index** | Option<**i32**> | Specifies the index of the first result. First item is number 1. |  |
**count** | Option<**i32**> | Desired number of results per page. Default is 10000. |  |
**sort_by** | Option<**String**> | Attribute to sort the results. |  |
**sort_order** | Option<**String**> | The order to sort the results. |  |

### Return type

[**crate::models::IamListGroupsResponse**](IamListGroupsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_groupspatch

> serde_json::Value account_groupspatch(account_id, id, iam_partial_update)
Update group details

Partially updates the details of a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Databricks account ID | [required] |
**id** | [**serde_json::Value**](.md) | Unique ID for a group in the Databricks account. | [required] |
**iam_partial_update** | [**IamPartialUpdate**](IamPartialUpdate.md) | Operations to be applied on group information. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_groupsupdate

> serde_json::Value account_groupsupdate(account_id, id, iam_group)
Replace a group

Updates the details of a group by replacing the entire group entity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Databricks account ID | [required] |
**id** | [**serde_json::Value**](.md) | Unique ID for a group in the Databricks account. | [required] |
**iam_group** | [**IamGroup**](IamGroup.md) | Operations to be applied on group information. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

