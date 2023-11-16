# \GroupsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**groupscreate**](GroupsApi.md#groupscreate) | **POST** /api/2.0/preview/scim/v2/Groups | Create a new group
[**groupsdelete**](GroupsApi.md#groupsdelete) | **DELETE** /api/2.0/preview/scim/v2/Groups/{id} | Delete a group
[**groupsget**](GroupsApi.md#groupsget) | **GET** /api/2.0/preview/scim/v2/Groups/{id} | Get group details
[**groupslist**](GroupsApi.md#groupslist) | **GET** /api/2.0/preview/scim/v2/Groups | List group details
[**groupspatch**](GroupsApi.md#groupspatch) | **PATCH** /api/2.0/preview/scim/v2/Groups/{id} | Update group details
[**groupsupdate**](GroupsApi.md#groupsupdate) | **PUT** /api/2.0/preview/scim/v2/Groups/{id} | Replace a group



## groupscreate

> crate::models::IamGroup groupscreate(iam_group)
Create a new group

Creates a group in the Databricks workspace with a unique name, using the supplied group details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iam_group** | [**IamGroup**](IamGroup.md) | Properties of the new group. | [required] |

### Return type

[**crate::models::IamGroup**](IamGroup.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groupsdelete

> serde_json::Value groupsdelete(id)
Delete a group

Deletes a group from the Databricks workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | Unique ID for a group in the Databricks workspace. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groupsget

> crate::models::IamGroup groupsget(id)
Get group details

Gets the information for a specific group in the Databricks workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | Unique ID for a group in the Databricks workspace. | [required] |

### Return type

[**crate::models::IamGroup**](IamGroup.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groupslist

> crate::models::IamListGroupsResponse groupslist(filter, attributes, excluded_attributes, start_index, count, sort_by, sort_order)
List group details

Gets all details of the groups associated with the Databricks workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Query by which the results have to be filtered. Supported operators are equals(`eq`), contains(`co`), starts with(`sw`) and not equals(`ne`). Additionally, simple expressions can be formed using logical operators - `and` and `or`. The [SCIM RFC](https://Toolsietf.org/html/rfc7644#section-3.4.2.2) has more details but we currently only support simple expressions. |  |
**attributes** | Option<**String**> | Comma-separated list of attributes to return in response. |  |
**excluded_attributes** | Option<**String**> | Comma-separated list of attributes to exclude in response. |  |
**start_index** | Option<**i32**> | Specifies the index of the first result. First item is number 1. |  |
**count** | Option<**i32**> | Desired number of results per page. |  |
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


## groupspatch

> serde_json::Value groupspatch(id, iam_partial_update)
Update group details

Partially updates the details of a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | Unique ID for a group in the Databricks workspace. | [required] |
**iam_partial_update** | [**IamPartialUpdate**](IamPartialUpdate.md) | Operations to be applied on group information. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groupsupdate

> serde_json::Value groupsupdate(id, iam_group)
Replace a group

Updates the details of a group by replacing the entire group entity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | Unique ID for a group in the Databricks workspace. | [required] |
**iam_group** | [**IamGroup**](IamGroup.md) | Operations to be applied on group information. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

