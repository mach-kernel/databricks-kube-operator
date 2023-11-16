# \VolumesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**volumescreate**](VolumesApi.md#volumescreate) | **POST** /api/2.1/unity-catalog/volumes | Create a Volume
[**volumesdelete**](VolumesApi.md#volumesdelete) | **DELETE** /api/2.1/unity-catalog/volumes/{full_name_arg} | Delete a Volume
[**volumeslist**](VolumesApi.md#volumeslist) | **GET** /api/2.1/unity-catalog/volumes | List Volumes
[**volumesread**](VolumesApi.md#volumesread) | **GET** /api/2.1/unity-catalog/volumes/{full_name_arg} | Get a Volume
[**volumesupdate**](VolumesApi.md#volumesupdate) | **PATCH** /api/2.1/unity-catalog/volumes/{full_name_arg} | Update a Volume



## volumescreate

> crate::models::CatalogVolumeInfo volumescreate(catalog_create_volume_request_content)
Create a Volume

Creates a new volume.  The user could create either an external volume or a managed volume. An external volume will be created in the specified external location, while a managed volume will be located in the default location which is specified by the parent schema, or the parent catalog, or the Metastore.  For the volume creation to succeed, the user must satisfy following conditions: - The caller must be a metastore admin, or be the owner of the parent catalog and schema,   or have the **USE_CATALOG** privilege on the parent catalog   and the **USE_SCHEMA** privilege on the parent schema. - The caller must have **CREATE VOLUME** privilege on the parent schema.  For an external volume, following conditions also need to satisfy - The caller must have **CREATE EXTERNAL VOLUME** privilege on the external location. - There are no other tables, nor volumes existing in the specified storage location. - The specified storage location is not under the location of other tables, nor volumes,   or catalogs or schemas. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_create_volume_request_content** | [**CatalogCreateVolumeRequestContent**](CatalogCreateVolumeRequestContent.md) |  | [required] |

### Return type

[**crate::models::CatalogVolumeInfo**](CatalogVolumeInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volumesdelete

> volumesdelete(full_name_arg)
Delete a Volume

Deletes a volume from the specified parent catalog and schema.  The caller must be a metastore admin or an owner of the volume. For the latter case, the caller must also be the owner or have the **USE_CATALOG** privilege on the parent catalog and the **USE_SCHEMA** privilege on the parent schema. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name_arg** | **String** | The three-level (fully qualified) name of the volume | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volumeslist

> crate::models::CatalogListVolumesResponseContent volumeslist(catalog_name, schema_name)
List Volumes

Gets an array of all volumes for the current metastore under the parent catalog and schema.  The returned volumes are filtered based on the privileges of the calling user. For example, the metastore admin is able to list all the volumes. A regular user needs to be the owner or have the **READ VOLUME** privilege on the volume to recieve the volumes in the response. For the latter case, the caller must also be the owner or have the **USE_CATALOG** privilege on the parent catalog and the **USE_SCHEMA** privilege on the parent schema.  There is no guarantee of a specific ordering of the elements in the array. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_name** | **String** | The identifier of the catalog | [required] |
**schema_name** | **String** | The identifier of the schema | [required] |

### Return type

[**crate::models::CatalogListVolumesResponseContent**](CatalogListVolumesResponseContent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volumesread

> crate::models::CatalogVolumeInfo volumesread(full_name_arg)
Get a Volume

Gets a volume from the metastore for a specific catalog and schema.  The caller must be a metastore admin or an owner of (or have the **READ VOLUME** privilege on) the volume. For the latter case, the caller must also be the owner or have the **USE_CATALOG** privilege on the parent catalog and the **USE_SCHEMA** privilege on the parent schema. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name_arg** | **String** | The three-level (fully qualified) name of the volume | [required] |

### Return type

[**crate::models::CatalogVolumeInfo**](CatalogVolumeInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## volumesupdate

> crate::models::CatalogVolumeInfo volumesupdate(full_name_arg, catalog_update_volume_request_content)
Update a Volume

Updates the specified volume under the specified parent catalog and schema.  The caller must be a metastore admin or an owner of the volume. For the latter case, the caller must also be the owner or have the **USE_CATALOG** privilege on the parent catalog and the **USE_SCHEMA** privilege on the parent schema.  Currently only the name, the owner or the comment of the volume could be updated. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full_name_arg** | **String** | The three-level (fully qualified) name of the volume | [required] |
**catalog_update_volume_request_content** | Option<[**CatalogUpdateVolumeRequestContent**](CatalogUpdateVolumeRequestContent.md)> |  |  |

### Return type

[**crate::models::CatalogVolumeInfo**](CatalogVolumeInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

