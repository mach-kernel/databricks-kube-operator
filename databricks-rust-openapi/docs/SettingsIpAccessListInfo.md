# SettingsIpAccessListInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**list_id** | Option<[**String**](String.md)> | Universally unique identifier (UUID) of the IP access list. | [optional]
**address_count** | Option<**i32**> | Total number of IP or CIDR values. | [optional]
**label** | Option<**String**> | Label for the IP access list. This **cannot** be empty. | [optional]
**enabled** | Option<**bool**> | Specifies whether this IP access list is enabled. | [optional]
**created_by** | Option<**i64**> | User ID of the user who created this list. | [optional]
**list_type** | Option<[**crate::models::SettingslistType**](Settingslist_type.md)> |  | [optional]
**updated_at** | Option<**i64**> | Update timestamp in milliseconds. | [optional]
**ip_addresses** | Option<**Vec<String>**> |  | [optional]
**created_at** | Option<**i64**> | Creation timestamp in milliseconds. | [optional]
**updated_by** | Option<**i64**> | User ID of the user who updated this list. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


