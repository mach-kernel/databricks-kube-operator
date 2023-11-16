# \CleanRoomsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clean_roomscreate**](CleanRoomsApi.md#clean_roomscreate) | **POST** /api/2.1/unity-catalog/clean-rooms | Create a clean room
[**clean_roomsdelete**](CleanRoomsApi.md#clean_roomsdelete) | **DELETE** /api/2.1/unity-catalog/clean-rooms/{name_arg} | Delete a clean room
[**clean_roomsget**](CleanRoomsApi.md#clean_roomsget) | **GET** /api/2.1/unity-catalog/clean-rooms/{name_arg} | Get a clean room
[**clean_roomslist**](CleanRoomsApi.md#clean_roomslist) | **GET** /api/2.1/unity-catalog/clean-rooms | List clean rooms
[**clean_roomsupdate**](CleanRoomsApi.md#clean_roomsupdate) | **PATCH** /api/2.1/unity-catalog/clean-rooms/{name_arg} | Update a clean room



## clean_roomscreate

> crate::models::SharingCleanRoomInfo clean_roomscreate(sharing_create_clean_room)
Create a clean room

Creates a new clean room with specified colaborators. The caller must be a metastore admin or have the **CREATE_CLEAN_ROOM** privilege on the metastore. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_create_clean_room** | Option<[**SharingCreateCleanRoom**](SharingCreateCleanRoom.md)> |  |  |

### Return type

[**crate::models::SharingCleanRoomInfo**](SharingCleanRoomInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clean_roomsdelete

> serde_json::Value clean_roomsdelete(name_arg)
Delete a clean room

Deletes a data object clean room from the metastore. The caller must be an owner of the clean room. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name_arg** | **String** | The name of the clean room. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clean_roomsget

> crate::models::SharingCleanRoomInfo clean_roomsget(name_arg, include_remote_details)
Get a clean room

Gets a data object clean room from the metastore. The caller must be a metastore admin or the owner of the clean room. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name_arg** | **String** | The name of the clean room. | [required] |
**include_remote_details** | Option<**bool**> | Whether to include remote details (central) on the clean room. |  |

### Return type

[**crate::models::SharingCleanRoomInfo**](SharingCleanRoomInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clean_roomslist

> crate::models::SharingListCleanRoomsResponse clean_roomslist()
List clean rooms

Gets an array of data object clean rooms from the metastore. The caller must be a metastore admin or the owner of the clean room. There is no guarantee of a specific ordering of the elements in the array. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SharingListCleanRoomsResponse**](SharingListCleanRoomsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clean_roomsupdate

> crate::models::SharingCleanRoomInfo clean_roomsupdate(name_arg, sharing_update_clean_room)
Update a clean room

Updates the clean room with the changes and data objects in the request. The caller must be the owner of the clean room or a metastore admin.  When the caller is a metastore admin, only the __owner__ field can be updated.  In the case that the clean room name is changed **updateCleanRoom** requires that the caller is both the clean room owner and a metastore admin.  For each table that is added through this method, the clean room owner must also have **SELECT** privilege on the table. The privilege must be maintained indefinitely for recipients to be able to access the table. Typically, you should use a group as the clean room owner.  Table removals through **update** do not require additional privileges. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name_arg** | **String** | The name of the clean room. | [required] |
**sharing_update_clean_room** | Option<[**SharingUpdateCleanRoom**](SharingUpdateCleanRoom.md)> |  |  |

### Return type

[**crate::models::SharingCleanRoomInfo**](SharingCleanRoomInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

