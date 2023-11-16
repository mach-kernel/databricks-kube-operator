# \ArtifactAllowlistsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**artifact_allowlistsget**](ArtifactAllowlistsApi.md#artifact_allowlistsget) | **GET** /api/2.1/unity-catalog/artifact-allowlists/{artifact_type} | Get an artifact allowlist
[**artifact_allowlistsupdate**](ArtifactAllowlistsApi.md#artifact_allowlistsupdate) | **PUT** /api/2.1/unity-catalog/artifact-allowlists/{artifact_type} | Set an artifact allowlist



## artifact_allowlistsget

> crate::models::CatalogArtifactAllowlistInfo artifact_allowlistsget(artifact_type)
Get an artifact allowlist

Get the artifact allowlist of a certain artifact type. The caller must be a metastore admin. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**artifact_type** | [**CatalogArtifactType**](.md) | The artifact type of the allowlist. | [required] |

### Return type

[**crate::models::CatalogArtifactAllowlistInfo**](CatalogArtifactAllowlistInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## artifact_allowlistsupdate

> crate::models::CatalogArtifactAllowlistInfo artifact_allowlistsupdate(artifact_type, catalog_set_artifact_allowlist)
Set an artifact allowlist

Set the artifact allowlist of a certain artifact type. The whole artifact allowlist is replaced with the new allowlist. The caller must be a metastore admin. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**artifact_type** | [**CatalogArtifactType**](.md) | The artifact type of the allowlist. | [required] |
**catalog_set_artifact_allowlist** | Option<[**CatalogSetArtifactAllowlist**](CatalogSetArtifactAllowlist.md)> |  |  |

### Return type

[**crate::models::CatalogArtifactAllowlistInfo**](CatalogArtifactAllowlistInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

