# SharingCleanRoomInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**owner** | Option<**String**> | Username of current owner of clean room. | [optional]
**remote_detailed_info** | Option<[**crate::models::SharingCentralCleanRoomInfo**](SharingCentralCleanRoomInfo.md)> | Central clean room details. | [optional]
**local_catalogs** | Option<[**Vec<crate::models::SharingCleanRoomCatalog>**](SharingCleanRoomCatalog.md)> |  | [optional]
**created_by** | Option<**String**> | Username of clean room creator. | [optional][readonly]
**name** | Option<**String**> | Name of the clean room. | [optional]
**updated_at** | Option<**i64**> | Time at which this clean room was updated, in epoch milliseconds. | [optional][readonly]
**created_at** | Option<**i64**> | Time at which this clean room was created, in epoch milliseconds. | [optional][readonly]
**comment** | Option<**String**> | User-provided free-form text description. | [optional]
**updated_by** | Option<**String**> | Username of clean room updater. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


