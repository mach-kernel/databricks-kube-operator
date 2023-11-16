# SharingProviderInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**owner** | Option<**String**> | Username of Provider owner. | [optional]
**cloud** | Option<**String**> | Cloud vendor of the provider's UC metastore. This field is only present when the __authentication_type__ is **DATABRICKS**. | [optional][readonly]
**data_provider_global_metastore_id** | Option<**String**> | The global UC metastore id of the data provider. This field is only present when the __authentication_type__ is **DATABRICKS**. The identifier is of format <cloud>:<region>:<metastore-uuid>. | [optional]
**metastore_id** | Option<**String**> | UUID of the provider's UC metastore. This field is only present when the __authentication_type__ is **DATABRICKS**. | [optional][readonly]
**created_by** | Option<**String**> | Username of Provider creator. | [optional][readonly]
**name** | Option<**String**> | The name of the Provider. | [optional]
**authentication_type** | Option<[**crate::models::SharingAuthenticationType**](SharingAuthenticationType.md)> |  | [optional]
**updated_at** | Option<**i64**> | Time at which this Provider was created, in epoch milliseconds. | [optional][readonly]
**region** | Option<**String**> | Cloud region of the provider's UC metastore. This field is only present when the __authentication_type__ is **DATABRICKS**. | [optional][readonly]
**recipient_profile** | Option<[**crate::models::SharingRecipientProfile**](SharingRecipientProfile.md)> | The recipient profile. This field is only present when the authentication_type is `TOKEN`. | [optional]
**created_at** | Option<**i64**> | Time at which this Provider was created, in epoch milliseconds. | [optional][readonly]
**comment** | Option<**String**> | Description about the provider. | [optional]
**updated_by** | Option<**String**> | Username of user who last modified Share. | [optional][readonly]
**recipient_profile_str** | Option<**String**> | This field is only present when the authentication_type is `TOKEN` or not provided. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


