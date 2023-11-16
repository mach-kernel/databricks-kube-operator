# \RecipientsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**recipientscreate**](RecipientsApi.md#recipientscreate) | **POST** /api/2.1/unity-catalog/recipients | Create a share recipient
[**recipientsdelete**](RecipientsApi.md#recipientsdelete) | **DELETE** /api/2.1/unity-catalog/recipients/{name} | Delete a share recipient
[**recipientsget**](RecipientsApi.md#recipientsget) | **GET** /api/2.1/unity-catalog/recipients/{name} | Get a share recipient
[**recipientslist**](RecipientsApi.md#recipientslist) | **GET** /api/2.1/unity-catalog/recipients | List share recipients
[**recipientsrotate_token**](RecipientsApi.md#recipientsrotate_token) | **POST** /api/2.1/unity-catalog/recipients/{name}/rotate-token | Rotate a token
[**recipientsshare_permissions**](RecipientsApi.md#recipientsshare_permissions) | **GET** /api/2.1/unity-catalog/recipients/{name}/share-permissions | Get recipient share permissions
[**recipientsupdate**](RecipientsApi.md#recipientsupdate) | **PATCH** /api/2.1/unity-catalog/recipients/{name} | Update a share recipient



## recipientscreate

> crate::models::SharingRecipientInfo recipientscreate(sharing_create_recipient)
Create a share recipient

Creates a new recipient with the delta sharing authentication type in the metastore. The caller must be a metastore admin or has the **CREATE_RECIPIENT** privilege on the metastore. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_create_recipient** | Option<[**SharingCreateRecipient**](SharingCreateRecipient.md)> |  |  |

### Return type

[**crate::models::SharingRecipientInfo**](SharingRecipientInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recipientsdelete

> serde_json::Value recipientsdelete(name)
Delete a share recipient

Deletes the specified recipient from the metastore. The caller must be the owner of the recipient.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the recipient. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recipientsget

> crate::models::SharingRecipientInfo recipientsget(name)
Get a share recipient

Gets a share recipient from the metastore if:    * the caller is the owner of the share recipient, or:   * is a metastore admin 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the recipient. | [required] |

### Return type

[**crate::models::SharingRecipientInfo**](SharingRecipientInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recipientslist

> crate::models::SharingListRecipientsResponse recipientslist(data_recipient_global_metastore_id)
List share recipients

Gets an array of all share recipients within the current metastore where:    * the caller is a metastore admin, or   * the caller is the owner. There is no guarantee of a specific ordering of the elements in the array. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_recipient_global_metastore_id** | Option<**String**> | If not provided, all recipients will be returned. If no recipients exist with this ID, no results will be returned. |  |

### Return type

[**crate::models::SharingListRecipientsResponse**](SharingListRecipientsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recipientsrotate_token

> crate::models::SharingRecipientInfo recipientsrotate_token(name, sharing_rotate_recipient_token)
Rotate a token

Refreshes the specified recipient's delta sharing authentication token with the provided token info. The caller must be the owner of the recipient. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the recipient. | [required] |
**sharing_rotate_recipient_token** | Option<[**SharingRotateRecipientToken**](SharingRotateRecipientToken.md)> |  |  |

### Return type

[**crate::models::SharingRecipientInfo**](SharingRecipientInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recipientsshare_permissions

> crate::models::SharingGetRecipientSharePermissionsResponse recipientsshare_permissions(name)
Get recipient share permissions

Gets the share permissions for the specified Recipient. The caller must be a metastore admin or the owner of the Recipient.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the Recipient. | [required] |

### Return type

[**crate::models::SharingGetRecipientSharePermissionsResponse**](SharingGetRecipientSharePermissionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recipientsupdate

> serde_json::Value recipientsupdate(name, sharing_update_recipient)
Update a share recipient

Updates an existing recipient in the metastore. The caller must be a metastore admin or the owner of the recipient. If the recipient name will be updated, the user must be both a metastore admin and the owner of the recipient. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the recipient. | [required] |
**sharing_update_recipient** | Option<[**SharingUpdateRecipient**](SharingUpdateRecipient.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

