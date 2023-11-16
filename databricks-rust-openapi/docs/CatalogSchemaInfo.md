# CatalogSchemaInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**properties** | Option<**::std::collections::HashMap<String, String>**> | A map of key-value properties attached to the securable. | [optional]
**owner** | Option<**String**> | Username of current owner of schema. | [optional]
**catalog_name** | Option<**String**> | Name of parent catalog. | [optional]
**full_name** | Option<**String**> | Full name of schema, in form of __catalog_name__.__schema_name__. | [optional][readonly]
**effective_predictive_optimization_flag** | Option<[**crate::models::CatalogEffectivePredictiveOptimizationFlag**](CatalogEffectivePredictiveOptimizationFlag.md)> |  | [optional][readonly]
**metastore_id** | Option<**String**> | Unique identifier of parent metastore. | [optional][readonly]
**created_by** | Option<**String**> | Username of schema creator. | [optional][readonly]
**name** | Option<**String**> | Name of schema, relative to parent catalog. | [optional]
**updated_at** | Option<**i64**> | Time at which this schema was created, in epoch milliseconds. | [optional][readonly]
**enable_predictive_optimization** | Option<[**crate::models::CatalogEnablePredictiveOptimization**](CatalogEnablePredictiveOptimization.md)> |  | [optional]
**storage_root** | Option<**String**> | Storage root URL for managed tables within schema. | [optional]
**catalog_type** | Option<**String**> | The type of the parent catalog. | [optional][readonly]
**created_at** | Option<**i64**> | Time at which this schema was created, in epoch milliseconds. | [optional][readonly]
**comment** | Option<**String**> | User-provided free-form text description. | [optional]
**updated_by** | Option<**String**> | Username of user who last modified schema. | [optional][readonly]
**storage_location** | Option<**String**> | Storage location for managed tables within schema. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


