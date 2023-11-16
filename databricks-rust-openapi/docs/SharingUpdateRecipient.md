# SharingUpdateRecipient

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**comment** | Option<**String**> | Description about the recipient. | [optional]
**ip_access_list** | Option<[**crate::models::SharingIpAccessList**](SharingIpAccessList.md)> | IP Access List | [optional]
**name** | Option<**String**> | Name of Recipient. | [optional]
**owner** | Option<**String**> | Username of the recipient owner. | [optional]
**properties_kvpairs** | Option<[**crate::models::SharingSecurablePropertiesKvPairs**](SharingSecurablePropertiesKvPairs.md)> | Recipient properties as map of string key-value pairs. When provided in update request, the specified properties will override the existing properties. To add and remove properties, one would need to perform a read-modify-write.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


