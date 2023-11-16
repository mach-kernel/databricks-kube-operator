# MlCreateRegistryWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | User-specified description for the webhook. | [optional]
**events** | [**Vec<crate::models::MlRegistryWebhookEvent>**](MlRegistryWebhookEvent.md) |  | 
**http_url_spec** | Option<[**crate::models::MlHttpUrlSpec**](MlHttpUrlSpec.md)> |  | [optional]
**job_spec** | Option<[**crate::models::MlJobSpec**](MlJobSpec.md)> |  | [optional]
**model_name** | Option<**String**> | Name of the model whose events would trigger this webhook. | [optional]
**status** | Option<[**crate::models::MlRegistryWebhookStatus**](MlRegistryWebhookStatus.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


