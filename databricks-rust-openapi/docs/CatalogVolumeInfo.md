# CatalogVolumeInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**owner** | Option<**String**> | The identifier of the user who owns the volume | [optional]
**catalog_name** | Option<**String**> | The name of the catalog where the schema and the volume are | [optional]
**full_name** | Option<**String**> | The three-level (fully qualified) name of the volume | [optional][readonly]
**encryption_details** | Option<[**crate::models::CatalogEncryptionDetails**](CatalogEncryptionDetails.md)> |  | [optional][readonly]
**metastore_id** | Option<**String**> | The unique identifier of the metastore | [optional][readonly]
**created_by** | Option<**String**> | The identifier of the user who created the volume | [optional][readonly]
**schema_name** | Option<**String**> | The name of the schema where the volume is | [optional]
**name** | Option<**String**> | The name of the volume | [optional]
**updated_at** | Option<**i64**> |  | [optional][readonly]
**volume_id** | Option<**String**> | The unique identifier of the volume | [optional][readonly]
**access_point** | Option<**String**> | The AWS access point to use when accesing s3 for this external location. | [optional][readonly]
**created_at** | Option<**i64**> |  | [optional][readonly]
**volume_type** | Option<[**crate::models::CatalogVolumeType**](CatalogVolumeType.md)> |  | [optional]
**comment** | Option<**String**> | The comment attached to the volume | [optional]
**updated_by** | Option<**String**> | The identifier of the user who updated the volume last time | [optional][readonly]
**storage_location** | Option<**String**> | The storage location on the cloud | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


