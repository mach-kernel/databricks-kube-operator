# MlHttpUrlSpecWithoutSecret

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enable_ssl_verification** | Option<**bool**> | Enable/disable SSL certificate validation. Default is true. For self-signed certificates, this field must be false AND the destination server must disable certificate validation as well. For security purposes, it is encouraged to perform secret validation with the HMAC-encoded portion of the payload and acknowledge the risk associated with disabling hostname validation whereby it becomes more likely that requests can be maliciously routed to an unintended host. | [optional]
**url** | Option<**String**> | External HTTPS URL called on event trigger (by using a POST request). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


