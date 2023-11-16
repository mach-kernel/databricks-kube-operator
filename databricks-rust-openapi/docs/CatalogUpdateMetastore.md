# CatalogUpdateMetastore

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delta_sharing_organization_name** | Option<**String**> | The organization name of a Delta Sharing entity, to be used in Databricks-to-Databricks Delta Sharing as the official name. | [optional]
**delta_sharing_recipient_token_lifetime_in_seconds** | Option<**i64**> | The lifetime of delta sharing recipient token in seconds. | [optional]
**delta_sharing_scope** | Option<**String**> | The scope of Delta Sharing enabled for the metastore. | [optional]
**name** | Option<**String**> | The user-specified name of the metastore. | [optional]
**owner** | Option<**String**> | The owner of the metastore. | [optional]
**privilege_model_version** | Option<**String**> | Privilege model version of the metastore, of the form `Majorminor` (e.g., `1.0`). | [optional]
**storage_root_credential_id** | Option<**String**> | UUID of storage credential to access the metastore storage_root. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


