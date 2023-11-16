# CatalogTableInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**properties** | Option<**::std::collections::HashMap<String, String>**> | A map of key-value properties attached to the securable. | [optional]
**owner** | Option<**String**> | Username of current owner of table. | [optional]
**catalog_name** | Option<**String**> | Name of parent catalog. | [optional]
**view_definition** | Option<**String**> | View definition SQL (when __table_type__ is **VIEW**, **MATERIALIZED_VIEW**, or **STREAMING_TABLE**) | [optional]
**full_name** | Option<**String**> | Full name of table, in form of __catalog_name__.__schema_name__.__table_name__ | [optional][readonly]
**columns** | Option<[**Vec<crate::models::CatalogColumnInfo>**](CatalogColumnInfo.md)> |  | [optional]
**table_type** | Option<[**crate::models::CatalogTableType**](CatalogTableType.md)> |  | [optional]
**data_access_configuration_id** | Option<**String**> | Unique ID of the Data Access Configuration to use with the table data. | [optional]
**effective_predictive_optimization_flag** | Option<[**crate::models::CatalogEffectivePredictiveOptimizationFlag**](CatalogEffectivePredictiveOptimizationFlag.md)> |  | [optional][readonly]
**encryption_details** | Option<[**crate::models::CatalogEncryptionDetails**](CatalogEncryptionDetails.md)> |  | [optional][readonly]
**metastore_id** | Option<**String**> | Unique identifier of parent metastore. | [optional][readonly]
**sql_path** | Option<**String**> | List of schemes whose objects can be referenced without qualification. | [optional]
**table_constraints** | Option<[**crate::models::CatalogTableConstraintList**](CatalogTableConstraintList.md)> |  | [optional]
**created_by** | Option<**String**> | Username of table creator. | [optional][readonly]
**schema_name** | Option<**String**> | Name of parent schema relative to its parent catalog. | [optional]
**row_filter** | Option<[**crate::models::CatalogTableRowFilter**](CatalogTableRowFilter.md)> |  | [optional]
**name** | Option<**String**> | Name of table, relative to parent schema. | [optional]
**updated_at** | Option<**i64**> | Time at which this table was last modified, in epoch milliseconds. | [optional][readonly]
**table_id** | Option<**String**> | Name of table, relative to parent schema. | [optional][readonly]
**enable_predictive_optimization** | Option<[**crate::models::CatalogEnablePredictiveOptimization**](CatalogEnablePredictiveOptimization.md)> |  | [optional]
**delta_runtime_properties_kvpairs** | Option<[**crate::models::CatalogDeltaRuntimePropertiesKvPairs**](CatalogDeltaRuntimePropertiesKvPairs.md)> | Information pertaining to current state of the delta table. | [optional][readonly]
**deleted_at** | Option<**i64**> | Time at which this table was deleted, in epoch milliseconds. Field is omitted if table is not deleted. | [optional][readonly]
**access_point** | Option<**String**> | The AWS access point to use when accesing s3 for this external location. | [optional][readonly]
**created_at** | Option<**i64**> | Time at which this table was created, in epoch milliseconds. | [optional][readonly]
**comment** | Option<**String**> | User-provided free-form text description. | [optional]
**data_source_format** | Option<[**crate::models::CatalogDataSourceFormat**](CatalogDataSourceFormat.md)> |  | [optional]
**updated_by** | Option<**String**> | Username of user who last modified the table. | [optional][readonly]
**storage_location** | Option<**String**> | Storage root URL for table (for **MANAGED**, **EXTERNAL** tables) | [optional]
**storage_credential_name** | Option<**String**> | Name of the storage credential, when a storage credential is configured for use with this table. | [optional]
**view_dependencies** | Option<[**Vec<crate::models::CatalogDependency>**](CatalogDependency.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


