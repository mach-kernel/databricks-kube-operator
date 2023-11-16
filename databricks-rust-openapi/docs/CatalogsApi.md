# \CatalogsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**catalogscreate**](CatalogsApi.md#catalogscreate) | **POST** /api/2.1/unity-catalog/catalogs | Create a catalog
[**catalogsdelete**](CatalogsApi.md#catalogsdelete) | **DELETE** /api/2.1/unity-catalog/catalogs/{name} | Delete a catalog
[**catalogsget**](CatalogsApi.md#catalogsget) | **GET** /api/2.1/unity-catalog/catalogs/{name} | Get a catalog
[**catalogslist**](CatalogsApi.md#catalogslist) | **GET** /api/2.1/unity-catalog/catalogs | List catalogs
[**catalogsupdate**](CatalogsApi.md#catalogsupdate) | **PATCH** /api/2.1/unity-catalog/catalogs/{name} | Update a catalog



## catalogscreate

> crate::models::CatalogCatalogInfo catalogscreate(catalog_create_catalog)
Create a catalog

Creates a new catalog instance in the parent metastore if the caller is a metastore admin or has the **CREATE_CATALOG** privilege. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**catalog_create_catalog** | Option<[**CatalogCreateCatalog**](CatalogCreateCatalog.md)> |  |  |

### Return type

[**crate::models::CatalogCatalogInfo**](CatalogCatalogInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## catalogsdelete

> serde_json::Value catalogsdelete(name, force)
Delete a catalog

Deletes the catalog that matches the supplied name. The caller must be a metastore admin or the owner of the catalog. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the catalog. | [required] |
**force** | Option<**bool**> | Force deletion even if the catalog is not empty. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## catalogsget

> crate::models::CatalogCatalogInfo catalogsget(name)
Get a catalog

Gets the specified catalog in a metastore. The caller must be a metastore admin, the owner of the catalog, or a user that has the **USE_CATALOG** privilege set for their account. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the catalog. | [required] |

### Return type

[**crate::models::CatalogCatalogInfo**](CatalogCatalogInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## catalogslist

> crate::models::CatalogListCatalogsResponse catalogslist()
List catalogs

Gets an array of catalogs in the metastore. If the caller is the metastore admin, all catalogs will be retrieved. Otherwise, only catalogs owned by the caller (or for which the caller has the **USE_CATALOG** privilege) will be retrieved. There is no guarantee of a specific ordering of the elements in the array. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CatalogListCatalogsResponse**](CatalogListCatalogsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## catalogsupdate

> crate::models::CatalogCatalogInfo catalogsupdate(name, catalog_update_catalog)
Update a catalog

Updates the catalog that matches the supplied name. The caller must be either the owner of the catalog, or a metastore admin (when changing the owner field of the catalog). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the catalog. | [required] |
**catalog_update_catalog** | Option<[**CatalogUpdateCatalog**](CatalogUpdateCatalog.md)> |  |  |

### Return type

[**crate::models::CatalogCatalogInfo**](CatalogCatalogInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

