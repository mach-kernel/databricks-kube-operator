# CatalogExternalLocationInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**owner** | Option<**String**> | The owner of the external location. | [optional]
**url** | Option<**String**> | Path URL of the external location. | [optional]
**credential_id** | Option<**String**> | Unique ID of the location's storage credential. | [optional][readonly]
**encryption_details** | Option<[**crate::models::CatalogEncryptionDetails**](CatalogEncryptionDetails.md)> |  | [optional]
**metastore_id** | Option<**String**> | Unique identifier of metastore hosting the external location. | [optional][readonly]
**created_by** | Option<**String**> | Username of external location creator. | [optional][readonly]
**read_only** | Option<**bool**> | Indicates whether the external location is read-only. | [optional]
**name** | Option<**String**> | Name of the external location. | [optional]
**updated_at** | Option<**i64**> | Time at which external location this was last modified, in epoch milliseconds. | [optional][readonly]
**credential_name** | Option<**String**> | Name of the storage credential used with this location. | [optional]
**access_point** | Option<**String**> | The AWS access point to use when accesing s3 for this external location. | [optional]
**created_at** | Option<**i64**> | Time at which this external location was created, in epoch milliseconds. | [optional][readonly]
**comment** | Option<**String**> | User-provided free-form text description. | [optional]
**updated_by** | Option<**String**> | Username of user who last modified the external location. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


