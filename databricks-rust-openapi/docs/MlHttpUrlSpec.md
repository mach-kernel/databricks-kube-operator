# MlHttpUrlSpec

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authorization** | Option<**String**> | Value of the authorization header that should be sent in the request sent by the wehbook. It should be of the form `\"<auth type> <credentials>\"`. If set to an empty string, no authorization header will be included in the request. | [optional]
**enable_ssl_verification** | Option<**bool**> | Enable/disable SSL certificate validation. Default is true. For self-signed certificates, this field must be false AND the destination server must disable certificate validation as well. For security purposes, it is encouraged to perform secret validation with the HMAC-encoded portion of the payload and acknowledge the risk associated with disabling hostname validation whereby it becomes more likely that requests can be maliciously routed to an unintended host. | [optional]
**secret** | Option<**String**> | Shared secret required for HMAC encoding payload. The HMAC-encoded payload will be sent in the header as: { \"X-Databricks-Signature\": $encoded_payload }. | [optional]
**url** | **String** | External HTTPS URL called on event trigger (by using a POST request). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


