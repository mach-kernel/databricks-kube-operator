# MlRegistryWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Webhook ID | [optional]
**job_spec** | Option<[**crate::models::MlJobSpecWithoutSecret**](MlJobSpecWithoutSecret.md)> |  | [optional]
**model_name** | Option<**String**> | Name of the model whose events would trigger this webhook. | [optional]
**status** | Option<[**crate::models::MlRegistryWebhookStatus**](MlRegistryWebhookStatus.md)> |  | [optional]
**events** | Option<[**Vec<crate::models::MlRegistryWebhookEvent>**](MlRegistryWebhookEvent.md)> |  | [optional]
**last_updated_timestamp** | Option<**i64**> | Time of the object at last update, as a Unix timestamp in milliseconds. | [optional]
**creation_timestamp** | Option<**i64**> | Creation time of the object, as a Unix timestamp in milliseconds. | [optional]
**description** | Option<**String**> | User-specified description for the webhook. | [optional]
**http_url_spec** | Option<[**crate::models::MlHttpUrlSpecWithoutSecret**](MlHttpUrlSpecWithoutSecret.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


