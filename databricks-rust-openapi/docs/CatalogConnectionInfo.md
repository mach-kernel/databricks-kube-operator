# CatalogConnectionInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**properties** | Option<**::std::collections::HashMap<String, String>**> | An object containing map of key-value properties attached to the connection.  | [optional]
**owner** | Option<**String**> | Username of current owner of the connection. | [optional]
**url** | Option<**String**> | URL of the remote data source, extracted from options. | [optional][readonly]
**full_name** | Option<**String**> | Full name of connection. | [optional][readonly]
**securable_kind** | Option<**String**> | Kind of connection securable. | [optional][readonly]
**metastore_id** | Option<**String**> | Unique identifier of parent metastore. | [optional][readonly]
**created_by** | Option<**String**> | Username of connection creator. | [optional][readonly]
**read_only** | Option<**bool**> | If the connection is read only. | [optional]
**connection_type** | Option<[**crate::models::CatalogConnectionType**](CatalogConnectionType.md)> |  | [optional]
**name** | Option<**String**> | Name of the connection. | [optional]
**connection_id** | Option<**String**> | Unique identifier of the Connection. | [optional][readonly]
**updated_at** | Option<**i64**> | Time at which this connection was updated, in epoch milliseconds. | [optional][readonly]
**credential_type** | Option<[**crate::models::CatalogCredentialType**](CatalogCredentialType.md)> |  | [optional][readonly]
**created_at** | Option<**i64**> | Time at which this connection was created, in epoch milliseconds. | [optional][readonly]
**securable_type** | Option<**String**> |  | [optional][readonly][default to CONNECTION]
**comment** | Option<**String**> | User-provided free-form text description. | [optional]
**updated_by** | Option<**String**> | Username of user who last modified connection. | [optional][readonly]
**options** | Option<**::std::collections::HashMap<String, String>**> | A map of key-value properties attached to the securable. | [optional]
**provisioning_state** | Option<[**crate::models::CatalogProvisioningState**](CatalogProvisioningState.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


