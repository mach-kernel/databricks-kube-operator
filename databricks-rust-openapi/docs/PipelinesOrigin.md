# PipelinesOrigin

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**materialization_name** | Option<**String**> | Materialization name. | [optional]
**pipeline_id** | Option<**String**> | The id of the pipeline. Globally unique. | [optional]
**cloud** | Option<**String**> | The cloud provider, Eg., AWS or Azure. | [optional]
**request_id** | Option<**String**> | The id of the request that caused an update. | [optional]
**uc_resource_id** | Option<**String**> | The Unity Catalog id of the MV or ST being updated. | [optional]
**host** | Option<**String**> | The optional host name where the event was triggered | [optional]
**dataset_name** | Option<**String**> | The name of a dataset. Unique within a pipeline. | [optional]
**org_id** | Option<**i32**> | The org id of the user. Unique within a cloud. | [optional]
**flow_name** | Option<**String**> | The name of the flow. Not unique. | [optional]
**region** | Option<**String**> | The cloud region. | [optional]
**table_id** | Option<**String**> | The id of a (delta) table. Globally unique. | [optional]
**maintenance_id** | Option<**String**> | The id of a maintenance run. Globally unique. | [optional]
**batch_id** | Option<**i32**> | The id of a batch. Unique within a flow. | [optional]
**cluster_id** | Option<**String**> | The id of the cluster where an execution happens. Unique within a region. | [optional]
**pipeline_name** | Option<**String**> | The name of the pipeline. Not unique. | [optional]
**flow_id** | Option<**String**> | The id of the flow. Globally unique. Incremental queries will generally  reuse the same id while complete queries will have a new id per update.  | [optional]
**update_id** | Option<**String**> | The id of an execution. Globally unique. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


