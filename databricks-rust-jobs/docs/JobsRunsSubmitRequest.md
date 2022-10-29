# JobsRunsSubmitRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tasks** | Option<[**Vec<crate::models::RunSubmitTaskSettings>**](RunSubmitTaskSettings.md)> |  | [optional]
**run_name** | Option<**String**> | An optional name for the run. The default value is `Untitled`. | [optional]
**git_source** | Option<[**crate::models::GitSource**](GitSource.md)> |  | [optional]
**timeout_seconds** | Option<**i32**> | An optional timeout applied to each run of this job. The default behavior is to have no timeout. | [optional]
**idempotency_token** | Option<**String**> | An optional token that can be used to guarantee the idempotency of job run requests. If a run with the provided token already exists, the request does not create a new run but returns the ID of the existing run instead. If a run with the provided token is deleted, an error is returned.  If you specify the idempotency token, upon failure you can retry until the request succeeds. Databricks guarantees that exactly one run is launched with that idempotency token.  This token must have at most 64 characters.  For more information, see [How to ensure idempotency for jobs](https://kb.databricks.com/jobs/jobs-idempotency.html). | [optional]
**access_control_list** | Option<[**Vec<crate::models::AccessControlRequest>**](AccessControlRequest.md)> | List of permissions to set on the job. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


