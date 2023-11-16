# CatalogUpdateExternalLocation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**owner** | Option<**String**> | The owner of the external location. | [optional]
**url** | Option<**String**> | Path URL of the external location. | [optional]
**encryption_details** | Option<[**crate::models::CatalogEncryptionDetails**](CatalogEncryptionDetails.md)> |  | [optional]
**force** | Option<**bool**> | Force update even if changing url invalidates dependent external tables or mounts. | [optional]
**read_only** | Option<**bool**> | Indicates whether the external location is read-only. | [optional]
**name** | Option<**String**> | Name of the external location. | [optional]
**credential_name** | Option<**String**> | Name of the storage credential used with this location. | [optional]
**access_point** | Option<**String**> | The AWS access point to use when accesing s3 for this external location. | [optional]
**comment** | Option<**String**> | User-provided free-form text description. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


