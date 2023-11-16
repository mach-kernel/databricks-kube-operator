# PipelinesPipelineEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | Option<**String**> | The display message associated with the event. | [optional]
**origin** | Option<[**crate::models::PipelinesOrigin**](PipelinesOrigin.md)> | Describes where the event originates from. | [optional]
**error** | Option<[**crate::models::PipelinesErrorDetail**](PipelinesErrorDetail.md)> | Information about an error captured by the event. | [optional]
**id** | Option<**String**> | A time-based, globally unique id. | [optional]
**timestamp** | Option<**String**> | The time of the event. | [optional]
**event_type** | Option<**String**> | The event type. Should always correspond to the details | [optional]
**level** | Option<[**crate::models::PipelinesEventLevel**](PipelinesEventLevel.md)> | The severity level of the event. | [optional]
**sequence** | Option<[**crate::models::PipelinesSequencing**](PipelinesSequencing.md)> | A sequencing object to identify and order events. | [optional]
**maturity_level** | Option<[**crate::models::PipelinesMaturityLevel**](PipelinesMaturityLevel.md)> | Maturity level for event_type. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


