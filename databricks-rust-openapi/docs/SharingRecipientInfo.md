# SharingRecipientInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**owner** | Option<**String**> | Username of the recipient owner. | [optional]
**cloud** | Option<**String**> | Cloud vendor of the recipient's Unity Catalog Metstore. This field is only present when the __authentication_type__ is **DATABRICKS**`. | [optional][readonly]
**data_recipient_global_metastore_id** | Option<[**serde_json::Value**](.md)> | The global Unity Catalog metastore id provided by the data recipient. This field is only present when the __authentication_type__ is **DATABRICKS**. The identifier is of format __cloud__:__region__:__metastore-uuid__.  | [optional]
**activation_url** | Option<**String**> | Full activation url to retrieve the access token. It will be empty if the token is already retrieved. | [optional][readonly]
**sharing_code** | Option<**String**> | The one-time sharing code provided by the data recipient. This field is only present when the __authentication_type__ is **DATABRICKS**. | [optional]
**metastore_id** | Option<**String**> | Unique identifier of recipient's Unity Catalog metastore. This field is only present when the __authentication_type__ is **DATABRICKS** | [optional][readonly]
**created_by** | Option<**String**> | Username of recipient creator. | [optional][readonly]
**name** | Option<**String**> | Name of Recipient. | [optional]
**authentication_type** | Option<[**crate::models::SharingAuthenticationType**](SharingAuthenticationType.md)> |  | [optional]
**updated_at** | Option<**i64**> | Time at which the recipient was updated, in epoch milliseconds. | [optional][readonly]
**region** | Option<**String**> | Cloud region of the recipient's Unity Catalog Metstore. This field is only present when the __authentication_type__ is **DATABRICKS**. | [optional][readonly]
**created_at** | Option<**i64**> | Time at which this recipient was created, in epoch milliseconds. | [optional][readonly]
**properties_kvpairs** | Option<[**crate::models::SharingSecurablePropertiesKvPairs**](SharingSecurablePropertiesKvPairs.md)> | Recipient properties as map of string key-value pairs.  | [optional]
**comment** | Option<**String**> | Description about the recipient. | [optional]
**updated_by** | Option<**String**> | Username of recipient updater. | [optional][readonly]
**ip_access_list** | Option<[**crate::models::SharingIpAccessList**](SharingIpAccessList.md)> | IP Access List | [optional]
**activated** | Option<**bool**> | A boolean status field showing whether the Recipient's activation URL has been exercised or not. | [optional][readonly]
**tokens** | Option<[**Vec<crate::models::SharingRecipientTokenInfo>**](SharingRecipientTokenInfo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


