# CatalogCreateExternalLocation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_point** | Option<**String**> | The AWS access point to use when accesing s3 for this external location. | [optional]
**comment** | Option<**String**> | User-provided free-form text description. | [optional]
**credential_name** | **String** | Name of the storage credential used with this location. | 
**encryption_details** | Option<[**crate::models::CatalogEncryptionDetails**](CatalogEncryptionDetails.md)> |  | [optional]
**name** | **String** | Name of the external location. | 
**read_only** | Option<**bool**> | Indicates whether the external location is read-only. | [optional]
**skip_validation** | Option<**bool**> | Skips validation of the storage credential associated with the external location. | [optional]
**url** | **String** | Path URL of the external location. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


