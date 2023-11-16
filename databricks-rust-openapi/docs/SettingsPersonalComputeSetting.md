# SettingsPersonalComputeSetting

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**etag** | Option<**String**> | etag used for versioning. The response is at least as fresh as the eTag provided. This is used for optimistic concurrency control as a way to help prevent simultaneous writes of a setting overwriting each other. It is strongly suggested that systems make use of the etag in the read -> update pattern to perform setting updates in order to avoid race conditions. That is, get an etag from a GET request, and pass it with the PATCH request to identify the setting version you are updating.  | [optional]
**personal_compute** | [**crate::models::SettingsPersonalComputeMessage**](SettingsPersonalComputeMessage.md) |  | 
**setting_name** | Option<**String**> | Name of the corresponding setting. Needs to be 'default' if there is only one setting instance per account. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


