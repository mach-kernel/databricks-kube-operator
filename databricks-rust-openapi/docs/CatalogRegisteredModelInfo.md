# CatalogRegisteredModelInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**owner** | Option<**String**> | The identifier of the user who owns the registered model | [optional]
**aliases** | Option<[**Vec<crate::models::CatalogRegisteredModelAlias>**](CatalogRegisteredModelAlias.md)> |  | [optional]
**catalog_name** | Option<**String**> | The name of the catalog where the schema and the registered model reside | [optional]
**full_name** | Option<**String**> | The three-level (fully qualified) name of the registered model | [optional][readonly]
**metastore_id** | Option<**String**> | The unique identifier of the metastore | [optional][readonly]
**created_by** | Option<**String**> | The identifier of the user who created the registered model | [optional][readonly]
**schema_name** | Option<**String**> | The name of the schema where the registered model resides | [optional]
**name** | Option<**String**> | The name of the registered model | [optional]
**updated_at** | Option<**i64**> | Last-update timestamp of the registered model in milliseconds since the Unix epoch | [optional][readonly]
**created_at** | Option<**i64**> | Creation timestamp of the registered model in milliseconds since the Unix epoch | [optional][readonly]
**comment** | Option<**String**> | The comment attached to the registered model | [optional]
**updated_by** | Option<**String**> | The identifier of the user who updated the registered model last time | [optional][readonly]
**storage_location** | Option<**String**> | The storage location on the cloud under which model version data files are stored | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


