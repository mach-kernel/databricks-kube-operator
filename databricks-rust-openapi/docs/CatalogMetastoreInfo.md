# CatalogMetastoreInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**owner** | Option<**String**> | The owner of the metastore. | [optional]
**cloud** | Option<**String**> | Cloud vendor of the metastore home shard (Eg., `aws`, `azure`, `gcp`). | [optional][readonly]
**delta_sharing_recipient_token_lifetime_in_seconds** | Option<**i64**> | The lifetime of delta sharing recipient token in seconds. | [optional]
**privilege_model_version** | Option<**String**> | Privilege model version of the metastore, of the form `Majorminor` (e.g., `1.0`). | [optional]
**metastore_id** | Option<**String**> | Unique identifier of metastore. | [optional][readonly]
**storage_root_credential_name** | Option<**String**> | Name of the storage credential to access the metastore storage_root. | [optional][readonly]
**created_by** | Option<**String**> | Username of metastore creator. | [optional][readonly]
**storage_root_credential_id** | Option<**String**> | UUID of storage credential to access the metastore storage_root. | [optional]
**name** | Option<**String**> | The user-specified name of the metastore. | [optional]
**global_metastore_id** | Option<**String**> | Globally unique metastore ID across clouds and regions, of the form `cloud:region:metastore_id`. | [optional][readonly]
**updated_at** | Option<**i64**> | Time at which the metastore was last modified, in epoch milliseconds. | [optional][readonly]
**region** | Option<**String**> | Cloud region which the metastore serves (Eg., `us-west-2`, `westus`). | [optional]
**default_data_access_config_id** | Option<**String**> | Unique identifier of the metastore's (Default) Data Access Configuration. | [optional]
**storage_root** | Option<**String**> | The storage root URL for metastore | [optional]
**created_at** | Option<**i64**> | Time at which this metastore was created, in epoch milliseconds. | [optional][readonly]
**delta_sharing_organization_name** | Option<**String**> | The organization name of a Delta Sharing entity, to be used in Databricks-to-Databricks Delta Sharing as the official name. | [optional]
**updated_by** | Option<**String**> | Username of user who last modified the metastore. | [optional][readonly]
**delta_sharing_scope** | Option<**String**> | The scope of Delta Sharing enabled for the metastore. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


