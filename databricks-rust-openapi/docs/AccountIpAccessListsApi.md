# \AccountIpAccessListsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_ip_access_listscreate**](AccountIpAccessListsApi.md#account_ip_access_listscreate) | **POST** /api/2.0/preview/accounts/{account_id}/ip-access-lists | Create access list
[**account_ip_access_listsdelete**](AccountIpAccessListsApi.md#account_ip_access_listsdelete) | **DELETE** /api/2.0/preview/accounts/{account_id}/ip-access-lists/{ip_access_list_id} | Delete access list
[**account_ip_access_listsget**](AccountIpAccessListsApi.md#account_ip_access_listsget) | **GET** /api/2.0/preview/accounts/{account_id}/ip-access-lists/{ip_access_list_id} | Get IP access list
[**account_ip_access_listslist**](AccountIpAccessListsApi.md#account_ip_access_listslist) | **GET** /api/2.0/preview/accounts/{account_id}/ip-access-lists | Get access lists
[**account_ip_access_listsreplace**](AccountIpAccessListsApi.md#account_ip_access_listsreplace) | **PUT** /api/2.0/preview/accounts/{account_id}/ip-access-lists/{ip_access_list_id} | Replace access list
[**account_ip_access_listsupdate**](AccountIpAccessListsApi.md#account_ip_access_listsupdate) | **PATCH** /api/2.0/preview/accounts/{account_id}/ip-access-lists/{ip_access_list_id} | Update access list



## account_ip_access_listscreate

> crate::models::SettingsCreateIpAccessListResponse account_ip_access_listscreate(account_id, settings_create_ip_access_list)
Create access list

Creates an IP access list for the account.  A list can be an allow list or a block list. See the top of this file for a description of how the server treats allow lists and block lists at runtime.  When creating or updating an IP access list:    * For all allow lists and block lists combined, the API supports a maximum of 1000   IP/CIDR values, where one CIDR counts as a single value. Attempts to exceed that number   return error 400 with `error_code` value `QUOTA_EXCEEDED`.   * If the new list would block the calling user's current IP, error 400 is returned with   `error_code` value `INVALID_STATE`.  It can take a few minutes for the changes to take effect. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of the account being managed. | [required] |
**settings_create_ip_access_list** | [**SettingsCreateIpAccessList**](SettingsCreateIpAccessList.md) | Details required to configure a block list or allow list. | [required] |

### Return type

[**crate::models::SettingsCreateIpAccessListResponse**](SettingsCreateIpAccessListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_ip_access_listsdelete

> account_ip_access_listsdelete(account_id, ip_access_list_id)
Delete access list

Deletes an IP access list, specified by its list ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of the account being managed. | [required] |
**ip_access_list_id** | [**serde_json::Value**](.md) | The ID for the corresponding IP access list. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_ip_access_listsget

> crate::models::SettingsGetIpAccessListResponse account_ip_access_listsget(ip_access_list_id, account_id, ip_access_list_id2)
Get IP access list

Gets an IP access list, specified by its list ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ip_access_list_id** | **String** |  | [required] |
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of the account being managed. | [required] |
**ip_access_list_id2** | [**serde_json::Value**](.md) | The ID for the corresponding IP access list. | [required] |

### Return type

[**crate::models::SettingsGetIpAccessListResponse**](SettingsGetIpAccessListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_ip_access_listslist

> crate::models::SettingsGetIpAccessListsResponse account_ip_access_listslist(account_id)
Get access lists

Gets all IP access lists for the specified account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of the account being managed. | [required] |

### Return type

[**crate::models::SettingsGetIpAccessListsResponse**](SettingsGetIpAccessListsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_ip_access_listsreplace

> serde_json::Value account_ip_access_listsreplace(account_id, ip_access_list_id, settings_replace_ip_access_list)
Replace access list

Replaces an IP access list, specified by its ID.  A list can include allow lists and block lists. See the top of this file for a description of how the server treats allow lists and block lists at run time. When replacing an IP access list:  * For all allow lists and block lists combined, the API supports a maximum of 1000 IP/CIDR values,    where one CIDR counts as a single value. Attempts to exceed that number return error 400 with `error_code`    value `QUOTA_EXCEEDED`.  * If the resulting list would block the calling user's current IP, error 400 is returned with `error_code`    value `INVALID_STATE`. It can take a few minutes for the changes to take effect. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of the account being managed. | [required] |
**ip_access_list_id** | [**serde_json::Value**](.md) | The ID for the corresponding IP access list. | [required] |
**settings_replace_ip_access_list** | [**SettingsReplaceIpAccessList**](SettingsReplaceIpAccessList.md) | Details required to replace an IP access list | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_ip_access_listsupdate

> serde_json::Value account_ip_access_listsupdate(account_id, ip_access_list_id, settings_update_ip_access_list)
Update access list

Updates an existing IP access list, specified by its ID.  A list can include allow lists and block lists. See the top of this file for a description of how the server treats allow lists and block lists at run time.  When updating an IP access list:    * For all allow lists and block lists combined, the API supports a maximum of 1000   IP/CIDR values, where one CIDR counts as a single value. Attempts to exceed that number   return error 400 with `error_code` value `QUOTA_EXCEEDED`.   * If the updated list would block the calling user's current IP, error 400 is returned   with `error_code` value `INVALID_STATE`.  It can take a few minutes for the changes to take effect. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of the account being managed. | [required] |
**ip_access_list_id** | [**serde_json::Value**](.md) | The ID for the corresponding IP access list. | [required] |
**settings_update_ip_access_list** | [**SettingsUpdateIpAccessList**](SettingsUpdateIpAccessList.md) | Details required to update an IP access list. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

