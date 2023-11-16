# \IpAccessListsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ip_access_listscreate**](IpAccessListsApi.md#ip_access_listscreate) | **POST** /api/2.0/ip-access-lists | Create access list
[**ip_access_listsdelete**](IpAccessListsApi.md#ip_access_listsdelete) | **DELETE** /api/2.0/ip-access-lists/{ip_access_list_id} | Delete access list
[**ip_access_listsget**](IpAccessListsApi.md#ip_access_listsget) | **GET** /api/2.0/ip-access-lists/{ip_access_list_id} | Get access list
[**ip_access_listslist**](IpAccessListsApi.md#ip_access_listslist) | **GET** /api/2.0/ip-access-lists | Get access lists
[**ip_access_listsreplace**](IpAccessListsApi.md#ip_access_listsreplace) | **PUT** /api/2.0/ip-access-lists/{ip_access_list_id} | Replace access list
[**ip_access_listsupdate**](IpAccessListsApi.md#ip_access_listsupdate) | **PATCH** /api/2.0/ip-access-lists/{ip_access_list_id} | Update access list



## ip_access_listscreate

> crate::models::SettingsCreateIpAccessListResponse ip_access_listscreate(settings_create_ip_access_list)
Create access list

Creates an IP access list for this workspace.   A list can be an allow list or a block list. See the top of this file for a description of how the server treats allow lists and block lists at runtime.  When creating or updating an IP access list:    * For all allow lists and block lists combined, the API supports a maximum of 1000 IP/CIDR values,   where one CIDR counts as a single value. Attempts to exceed that number return error 400 with `error_code` value `QUOTA_EXCEEDED`.   * If the new list would block the calling user's current IP, error 400 is returned with `error_code` value `INVALID_STATE`.  It can take a few minutes for the changes to take effect. **Note**: Your new IP access list has no effect until you enable the feature. See :method:workspaceconf/setStatus 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**settings_create_ip_access_list** | [**SettingsCreateIpAccessList**](SettingsCreateIpAccessList.md) | Details required to configure a block list or allow list. | [required] |

### Return type

[**crate::models::SettingsCreateIpAccessListResponse**](SettingsCreateIpAccessListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ip_access_listsdelete

> ip_access_listsdelete(ip_access_list_id)
Delete access list

Deletes an IP access list, specified by its list ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ip_access_list_id** | **String** | The ID for the corresponding IP access list to modify. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ip_access_listsget

> crate::models::SettingsFetchIpAccessListResponse ip_access_listsget(ip_access_list_id)
Get access list

Gets an IP access list, specified by its list ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ip_access_list_id** | **String** | The ID for the corresponding IP access list to modify. | [required] |

### Return type

[**crate::models::SettingsFetchIpAccessListResponse**](SettingsFetchIpAccessListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ip_access_listslist

> crate::models::SettingsGetIpAccessListResponse ip_access_listslist()
Get access lists

Gets all IP access lists for the specified workspace.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SettingsGetIpAccessListResponse**](SettingsGetIpAccessListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ip_access_listsreplace

> serde_json::Value ip_access_listsreplace(ip_access_list_id, settings_replace_ip_access_list)
Replace access list

Replaces an IP access list, specified by its ID.   A list can include allow lists and block lists. See the top of this file for a description of how the server treats allow lists and block lists at run time. When replacing an IP access list:  * For all allow lists and block lists combined, the API supports a maximum of 1000 IP/CIDR values,    where one CIDR counts as a single value. Attempts to exceed that number return error 400 with `error_code`    value `QUOTA_EXCEEDED`.  * If the resulting list would block the calling user's current IP, error 400 is returned with `error_code`    value `INVALID_STATE`. It can take a few minutes for the changes to take effect. Note that your resulting IP access list has no effect until you enable the feature. See :method:workspaceconf/setStatus. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ip_access_list_id** | **String** | The ID for the corresponding IP access list to modify. | [required] |
**settings_replace_ip_access_list** | [**SettingsReplaceIpAccessList**](SettingsReplaceIpAccessList.md) | Details required to replace an IP access list | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ip_access_listsupdate

> serde_json::Value ip_access_listsupdate(ip_access_list_id, settings_update_ip_access_list)
Update access list

Updates an existing IP access list, specified by its ID.   A list can include allow lists and block lists.  See the top of this file for a description of how the server treats allow lists and block lists at run time.  When updating an IP access list:    * For all allow lists and block lists combined, the API supports a maximum of 1000 IP/CIDR values,    where one CIDR counts as a single value. Attempts to exceed that number return error 400 with `error_code` value `QUOTA_EXCEEDED`.   * If the updated list would block the calling user's current IP, error 400 is returned with `error_code` value `INVALID_STATE`.  It can take a few minutes for the changes to take effect. Note that your resulting IP access list has no effect until you enable  the feature. See :method:workspaceconf/setStatus. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ip_access_list_id** | **String** | The ID for the corresponding IP access list to modify. | [required] |
**settings_update_ip_access_list** | [**SettingsUpdateIpAccessList**](SettingsUpdateIpAccessList.md) | Details required to update an IP access list. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

