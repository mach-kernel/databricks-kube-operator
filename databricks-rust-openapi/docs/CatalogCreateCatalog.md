# CatalogCreateCatalog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**comment** | Option<**String**> | User-provided free-form text description. | [optional]
**connection_name** | Option<**String**> | The name of the connection to an external data source. | [optional]
**name** | **String** | Name of catalog. | 
**properties** | Option<**::std::collections::HashMap<String, String>**> | A map of key-value properties attached to the securable. | [optional]
**provider_name** | Option<**String**> | The name of delta sharing provider.  A Delta Sharing catalog is a catalog that is based on a Delta share on a remote sharing server.  | [optional]
**share_name** | Option<**String**> | The name of the share under the share provider. | [optional]
**storage_root** | Option<**String**> | Storage root URL for managed tables within catalog. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


