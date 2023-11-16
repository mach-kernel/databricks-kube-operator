# \AccountUsersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_userscreate**](AccountUsersApi.md#account_userscreate) | **POST** /api/2.0/accounts/{account_id}/scim/v2/Users | Create a new user
[**account_usersdelete**](AccountUsersApi.md#account_usersdelete) | **DELETE** /api/2.0/accounts/{account_id}/scim/v2/Users/{id} | Delete a user
[**account_usersget**](AccountUsersApi.md#account_usersget) | **GET** /api/2.0/accounts/{account_id}/scim/v2/Users/{id} | Get user details
[**account_userslist**](AccountUsersApi.md#account_userslist) | **GET** /api/2.0/accounts/{account_id}/scim/v2/Users | List users
[**account_userspatch**](AccountUsersApi.md#account_userspatch) | **PATCH** /api/2.0/accounts/{account_id}/scim/v2/Users/{id} | Update user details
[**account_usersupdate**](AccountUsersApi.md#account_usersupdate) | **PUT** /api/2.0/accounts/{account_id}/scim/v2/Users/{id} | Replace a user



## account_userscreate

> crate::models::IamUser account_userscreate(account_id, iam_user)
Create a new user

Creates a new user in the Databricks account. This new user will also be added to the Databricks account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID | [required] |
**iam_user** | [**IamUser**](IamUser.md) | Properties of the new user. | [required] |

### Return type

[**crate::models::IamUser**](IamUser.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_usersdelete

> serde_json::Value account_usersdelete(account_id, id)
Delete a user

Deletes a user. Deleting a user from a Databricks account also removes objects associated with the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID | [required] |
**id** | [**serde_json::Value**](.md) | Unique ID for a user in the Databricks account. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_usersget

> crate::models::IamUser account_usersget(account_id, id)
Get user details

Gets information for a specific user in Databricks account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID | [required] |
**id** | [**serde_json::Value**](.md) | Unique ID for a user in the Databricks account. | [required] |

### Return type

[**crate::models::IamUser**](IamUser.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_userslist

> crate::models::IamListUsersResponse account_userslist(account_id, filter, attributes, excluded_attributes, start_index, count, sort_by, sort_order)
List users

Gets details for all the users associated with a Databricks account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID | [required] |
**filter** | Option<**String**> | Query by which the results have to be filtered. Supported operators are equals(`eq`), contains(`co`), starts with(`sw`) and not equals(`ne`). Additionally, simple expressions can be formed using logical operators - `and` and `or`. The [SCIM RFC](https://Toolsietf.org/html/rfc7644#section-3.4.2.2) has more details but we currently only support simple expressions. |  |
**attributes** | Option<**String**> | Comma-separated list of attributes to return in response. |  |
**excluded_attributes** | Option<**String**> | Comma-separated list of attributes to exclude in response. |  |
**start_index** | Option<**i32**> | Specifies the index of the first result. First item is number 1. |  |
**count** | Option<**i32**> | Desired number of results per page. Default is 10000. |  |
**sort_by** | Option<**String**> | Attribute to sort the results. Multi-part paths are supported. For example, `userName`, `NamegivenName`, and `emails`. |  |
**sort_order** | Option<**String**> | The order to sort the results. |  |

### Return type

[**crate::models::IamListUsersResponse**](IamListUsersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_userspatch

> serde_json::Value account_userspatch(account_id, id, iam_partial_update)
Update user details

Partially updates a user resource by applying the supplied operations on specific user attributes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID | [required] |
**id** | [**serde_json::Value**](.md) | Unique ID for a user in the Databricks account. | [required] |
**iam_partial_update** | [**IamPartialUpdate**](IamPartialUpdate.md) | Operations to be applied on user information. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_usersupdate

> serde_json::Value account_usersupdate(account_id, id, iam_user)
Replace a user

Replaces a user's information with the data supplied in request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID | [required] |
**id** | [**serde_json::Value**](.md) | Unique ID for a user in the Databricks account. | [required] |
**iam_user** | [**IamUser**](IamUser.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

