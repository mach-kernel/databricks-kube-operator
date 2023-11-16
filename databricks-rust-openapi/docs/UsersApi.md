# \UsersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**userscreate**](UsersApi.md#userscreate) | **POST** /api/2.0/preview/scim/v2/Users | Create a new user
[**usersdelete**](UsersApi.md#usersdelete) | **DELETE** /api/2.0/preview/scim/v2/Users/{id} | Delete a user
[**usersget**](UsersApi.md#usersget) | **GET** /api/2.0/preview/scim/v2/Users/{id} | Get user details
[**usersget_password_permission_levels**](UsersApi.md#usersget_password_permission_levels) | **GET** /api/2.0/permissions/authorization/passwords/permissionLevels | Get password permission levels
[**usersget_password_permissions**](UsersApi.md#usersget_password_permissions) | **GET** /api/2.0/permissions/authorization/passwords | Get password permissions
[**userslist**](UsersApi.md#userslist) | **GET** /api/2.0/preview/scim/v2/Users | List users
[**userspatch**](UsersApi.md#userspatch) | **PATCH** /api/2.0/preview/scim/v2/Users/{id} | Update user details
[**usersset_password_permissions**](UsersApi.md#usersset_password_permissions) | **PUT** /api/2.0/permissions/authorization/passwords | Set password permissions
[**usersupdate**](UsersApi.md#usersupdate) | **PUT** /api/2.0/preview/scim/v2/Users/{id} | Replace a user
[**usersupdate_password_permissions**](UsersApi.md#usersupdate_password_permissions) | **PATCH** /api/2.0/permissions/authorization/passwords | Update password permissions



## userscreate

> crate::models::IamUser userscreate(iam_user)
Create a new user

Creates a new user in the Databricks workspace. This new user will also be added to the Databricks account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iam_user** | [**IamUser**](IamUser.md) | Properties of the new user. | [required] |

### Return type

[**crate::models::IamUser**](IamUser.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## usersdelete

> serde_json::Value usersdelete(id)
Delete a user

Deletes a user. Deleting a user from a Databricks workspace also removes objects associated with the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | Unique ID for a user in the Databricks workspace. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## usersget

> crate::models::IamUser usersget(id)
Get user details

Gets information for a specific user in Databricks workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | Unique ID for a user in the Databricks workspace. | [required] |

### Return type

[**crate::models::IamUser**](IamUser.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## usersget_password_permission_levels

> crate::models::IamGetPasswordPermissionLevelsResponse usersget_password_permission_levels()
Get password permission levels

Gets the permission levels that a user can have on an object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IamGetPasswordPermissionLevelsResponse**](IamGetPasswordPermissionLevelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## usersget_password_permissions

> crate::models::IamPasswordPermissions usersget_password_permissions()
Get password permissions

Gets the permissions of all passwords. Passwords can inherit permissions from their root object.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IamPasswordPermissions**](IamPasswordPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## userslist

> crate::models::IamListUsersResponse userslist(filter, attributes, excluded_attributes, start_index, count, sort_by, sort_order)
List users

Gets details for all the users associated with a Databricks workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Query by which the results have to be filtered. Supported operators are equals(`eq`), contains(`co`), starts with(`sw`) and not equals(`ne`). Additionally, simple expressions can be formed using logical operators - `and` and `or`. The [SCIM RFC](https://Toolsietf.org/html/rfc7644#section-3.4.2.2) has more details but we currently only support simple expressions. |  |
**attributes** | Option<**String**> | Comma-separated list of attributes to return in response. |  |
**excluded_attributes** | Option<**String**> | Comma-separated list of attributes to exclude in response. |  |
**start_index** | Option<**i32**> | Specifies the index of the first result. First item is number 1. |  |
**count** | Option<**i32**> | Desired number of results per page. |  |
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


## userspatch

> serde_json::Value userspatch(id, iam_partial_update)
Update user details

Partially updates a user resource by applying the supplied operations on specific user attributes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | Unique ID for a user in the Databricks workspace. | [required] |
**iam_partial_update** | [**IamPartialUpdate**](IamPartialUpdate.md) | Operations to be applied on user information. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## usersset_password_permissions

> crate::models::IamPasswordPermissions usersset_password_permissions(iam_password_permissions_request)
Set password permissions

Sets permissions on all passwords. Passwords can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iam_password_permissions_request** | Option<[**IamPasswordPermissionsRequest**](IamPasswordPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::IamPasswordPermissions**](IamPasswordPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## usersupdate

> serde_json::Value usersupdate(id, iam_user)
Replace a user

Replaces a user's information with the data supplied in request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**serde_json::Value**](.md) | Unique ID for a user in the Databricks workspace. | [required] |
**iam_user** | [**IamUser**](IamUser.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## usersupdate_password_permissions

> crate::models::IamPasswordPermissions usersupdate_password_permissions(iam_password_permissions_request)
Update password permissions

Updates the permissions on all passwords. Passwords can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iam_password_permissions_request** | Option<[**IamPasswordPermissionsRequest**](IamPasswordPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::IamPasswordPermissions**](IamPasswordPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

