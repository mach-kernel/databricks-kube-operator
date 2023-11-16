# SqlEndpointHealth

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**details** | Option<**String**> | Details about errors that are causing current degraded/failed status. | [optional]
**failure_reason** | Option<[**crate::models::SqlTerminationReason**](SqlTerminationReason.md)> | The reason for failure to bring up clusters for this warehouse. This is available when status is 'FAILED' and sometimes when it is DEGRADED. | [optional]
**message** | Option<**String**> | Deprecated. split into summary and details for security | [optional]
**status** | Option<[**crate::models::SqlStatus**](SqlStatus.md)> |  | [optional]
**summary** | Option<**String**> | A short summary of the health status in case of degraded/failed warehouses. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


