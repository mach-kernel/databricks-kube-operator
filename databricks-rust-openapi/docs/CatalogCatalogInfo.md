# CatalogCatalogInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**properties** | Option<**::std::collections::HashMap<String, String>**> | A map of key-value properties attached to the securable. | [optional]
**provider_name** | Option<**String**> | The name of delta sharing provider.  A Delta Sharing catalog is a catalog that is based on a Delta share on a remote sharing server.  | [optional]
**owner** | Option<**String**> | Username of current owner of catalog. | [optional]
**connection_name** | Option<**String**> | The name of the connection to an external data source. | [optional]
**effective_predictive_optimization_flag** | Option<[**crate::models::CatalogEffectivePredictiveOptimizationFlag**](CatalogEffectivePredictiveOptimizationFlag.md)> |  | [optional][readonly]
**share_name** | Option<**String**> | The name of the share under the share provider. | [optional]
**metastore_id** | Option<**String**> | Unique identifier of parent metastore. | [optional][readonly]
**created_by** | Option<**String**> | Username of catalog creator. | [optional][readonly]
**name** | Option<**String**> | Name of catalog. | [optional]
**updated_at** | Option<**i64**> | Time at which this catalog was last modified, in epoch milliseconds. | [optional][readonly]
**enable_predictive_optimization** | Option<[**crate::models::CatalogEnablePredictiveOptimization**](CatalogEnablePredictiveOptimization.md)> |  | [optional]
**storage_root** | Option<**String**> | Storage root URL for managed tables within catalog. | [optional]
**catalog_type** | Option<[**crate::models::CatalogCatalogType**](CatalogCatalogType.md)> |  | [optional][readonly]
**created_at** | Option<**i64**> | Time at which this catalog was created, in epoch milliseconds. | [optional][readonly]
**isolation_mode** | Option<[**crate::models::CatalogIsolationMode**](CatalogIsolationMode.md)> |  | [optional]
**comment** | Option<**String**> | User-provided free-form text description. | [optional]
**updated_by** | Option<**String**> | Username of user who last modified catalog. | [optional][readonly]
**options** | Option<**::std::collections::HashMap<String, String>**> | A map of key-value properties attached to the securable. | [optional]
**storage_location** | Option<**String**> | Storage Location URL (full path) for managed tables within catalog. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


