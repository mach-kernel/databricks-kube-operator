# \ExternalLocationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**external_locationscreate**](ExternalLocationsApi.md#external_locationscreate) | **POST** /api/2.1/unity-catalog/external-locations | Create an external location
[**external_locationsdelete**](ExternalLocationsApi.md#external_locationsdelete) | **DELETE** /api/2.1/unity-catalog/external-locations/{name} | Delete an external location
[**external_locationsget**](ExternalLocationsApi.md#external_locationsget) | **GET** /api/2.1/unity-catalog/external-locations/{name} | Get an external location
[**external_locationslist**](ExternalLocationsApi.md#external_locationslist) | **GET** /api/2.1/unity-catalog/external-locations | List external locations
[**external_locationsupdate**](ExternalLocationsApi.md#external_locationsupdate) | **PATCH** /api/2.1/unity-catalog/external-locations/{name} | Update an external location



## external_locationscreate

> crate::models::CatalogExternalLocationInfo external_locationscreate(catalog_create_external_location)
Create an external location

Creates a new external location entry in the metastore. The caller must be a metastore admin or have the **CREATE_EXTERNAL_LOCATION** privilege on both the metastore and the associated storage credential. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_create_external_location** | Option<[**CatalogCreateExternalLocation**](CatalogCreateExternalLocation.md)> |  |  |

### Return type

[**crate::models::CatalogExternalLocationInfo**](CatalogExternalLocationInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_locationsdelete

> serde_json::Value external_locationsdelete(name, force)
Delete an external location

Deletes the specified external location from the metastore. The caller must be the owner of the external location. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the external location. | [required] |
**force** | Option<**bool**> | Force deletion even if there are dependent external tables or mounts. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_locationsget

> crate::models::CatalogExternalLocationInfo external_locationsget(name)
Get an external location

Gets an external location from the metastore. The caller must be either a metastore admin, the owner of the external location, or a user that has some privilege on the external location. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the external location. | [required] |

### Return type

[**crate::models::CatalogExternalLocationInfo**](CatalogExternalLocationInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_locationslist

> crate::models::CatalogListExternalLocationsResponse external_locationslist()
List external locations

Gets an array of external locations (__ExternalLocationInfo__ objects) from the metastore. The caller must be a metastore admin, the owner of the external location, or a user that has some privilege on the external location. There is no guarantee of a specific ordering of the elements in the array. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CatalogListExternalLocationsResponse**](CatalogListExternalLocationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## external_locationsupdate

> crate::models::CatalogExternalLocationInfo external_locationsupdate(name, catalog_update_external_location)
Update an external location

Updates an external location in the metastore. The caller must be the owner of the external location, or be a metastore admin. In the second case, the admin can only update the name of the external location. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the external location. | [required] |
**catalog_update_external_location** | Option<[**CatalogUpdateExternalLocation**](CatalogUpdateExternalLocation.md)> |  |  |

### Return type

[**crate::models::CatalogExternalLocationInfo**](CatalogExternalLocationInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

