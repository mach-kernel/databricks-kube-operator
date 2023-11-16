# JobsSubmitRun

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tasks** | Option<[**Vec<crate::models::JobsSubmitTask>**](JobsSubmitTask.md)> |  | [optional]
**idempotency_token** | Option<**String**> | An optional token that can be used to guarantee the idempotency of job run requests. If a run with the provided token already exists, the request does not create a new run but returns the ID of the existing run instead. If a run with the provided token is deleted, an error is returned.  If you specify the idempotency token, upon failure you can retry until the request succeeds. Databricks guarantees that exactly one run is launched with that idempotency token.  This token must have at most 64 characters.  For more information, see [How to ensure idempotency for jobs]( https://Kbdatabricks.com/jobs/jobs-idempotency.html). | [optional]
**timeout_seconds** | Option<**i32**> | An optional timeout applied to each run of this job. The default behavior is to have no timeout. | [optional]
**health** | Option<[**crate::models::JobsJobsHealthRules**](JobsJobsHealthRules.md)> |  | [optional]
**email_notifications** | Option<[**crate::models::JobsJobEmailNotifications**](JobsJobEmailNotifications.md)> | An optional set of email addresses notified when the run begins or completes. The default behavior is to not send any emails. | [optional]
**webhook_notifications** | Option<[**crate::models::JobsWebhookNotifications**](JobsWebhookNotifications.md)> | A collection of system notification IDs to notify when the run begins or completes. The default behavior is to not send any system notifications. | [optional]
**notification_settings** | Option<[**crate::models::JobsJobNotificationSettings**](JobsJobNotificationSettings.md)> | Optional notification settings that are used when sending notifications to each of the `webhook_notifications` for this run. | [optional]
**git_source** | Option<[**crate::models::JobsGitSource**](JobsGitSource.md)> | An optional specification for a remote Git repository containing the source code used by tasks. Version-controlled source code is supported by notebook, dbt, Python script, and SQL File tasks.  If `git_source` is set, these tasks retrieve the file from the remote repository by default. However, this behavior can be overridden by setting `source` to `WORKSPACE` on the task.  Note: dbt and SQL File tasks support only version-controlled sources. If dbt or SQL File tasks are used, `git_source` must be defined on the job. | [optional]
**access_control_list** | Option<[**Vec<crate::models::IamAccessControlRequest>**](IamAccessControlRequest.md)> |  | [optional]
**run_name** | Option<**String**> | An optional name for the run. The default value is `Untitled`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


