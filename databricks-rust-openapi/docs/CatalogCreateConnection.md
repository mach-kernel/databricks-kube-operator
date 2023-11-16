# CatalogCreateConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**comment** | Option<**String**> | User-provided free-form text description. | [optional]
**connection_type** | [**crate::models::CatalogConnectionType**](CatalogConnectionType.md) |  | 
**name** | **String** | Name of the connection. | 
**options** | Option<**::std::collections::HashMap<String, String>**> | A map of key-value properties attached to the securable. | 
**owner** | Option<**String**> | Username of current owner of the connection. | [optional]
**properties** | Option<**::std::collections::HashMap<String, String>**> | An object containing map of key-value properties attached to the connection.  | [optional]
**read_only** | Option<**bool**> | If the connection is read only. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


