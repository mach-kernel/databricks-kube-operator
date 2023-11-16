# SharingCreateRecipient

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authentication_type** | [**crate::models::SharingAuthenticationType**](SharingAuthenticationType.md) |  | 
**comment** | Option<**String**> | Description about the recipient. | [optional]
**data_recipient_global_metastore_id** | Option<[**serde_json::Value**](.md)> | The global Unity Catalog metastore id provided by the data recipient. This field is required when the __authentication_type__ is **DATABRICKS**. The identifier is of format __cloud__:__region__:__metastore-uuid__.  | [optional]
**ip_access_list** | Option<[**crate::models::SharingIpAccessList**](SharingIpAccessList.md)> | IP Access List | [optional]
**name** | **String** | Name of Recipient. | 
**owner** | Option<**String**> | Username of the recipient owner. | [optional]
**properties_kvpairs** | Option<[**crate::models::SharingSecurablePropertiesKvPairs**](SharingSecurablePropertiesKvPairs.md)> | Recipient properties as map of string key-value pairs.  | [optional]
**sharing_code** | Option<**String**> | The one-time sharing code provided by the data recipient. This field is required when the __authentication_type__ is **DATABRICKS**. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


