# JobsCreateJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tasks** | Option<[**Vec<crate::models::JobsTask>**](JobsTask.md)> |  | [optional]
**job_clusters** | Option<[**Vec<crate::models::JobsJobCluster>**](JobsJobCluster.md)> |  | [optional]
**timeout_seconds** | Option<**i32**> | An optional timeout applied to each run of this job. The default behavior is to have no timeout. | [optional]
**health** | Option<[**crate::models::JobsJobsHealthRules**](JobsJobsHealthRules.md)> |  | [optional]
**tags** | Option<**::std::collections::HashMap<String, String>**> | A map of tags associated with the job. These are forwarded to the cluster as cluster tags for jobs clusters, and are subject to the same limitations as cluster tags. A maximum of 25 tags can be added to the job. | [optional][default to {}]
**run_as** | Option<[**crate::models::JobsJobRunAs**](JobsJobRunAs.md)> |  | [optional]
**name** | Option<**String**> | An optional name for the job. The maximum length is 4096 bytes in UTF-8 encoding. | [optional][default to Untitled]
**schedule** | Option<[**crate::models::JobsCronSchedule**](JobsCronSchedule.md)> | An optional periodic schedule for this job. The default behavior is that the job only runs when triggered by clicking “Run Now” in the Jobs UI or sending an API request to `runNow`. | [optional]
**email_notifications** | Option<[**crate::models::JobsJobEmailNotifications**](JobsJobEmailNotifications.md)> | An optional set of email addresses that is notified when runs of this job begin or complete as well as when this job is deleted. The default behavior is to not send any emails. | [optional]
**max_concurrent_runs** | Option<**i32**> | An optional maximum allowed number of concurrent runs of the job.  Set this value if you want to be able to execute multiple runs of the same job concurrently. This is useful for example if you trigger your job on a frequent schedule and want to allow consecutive runs to overlap with each other, or if you want to trigger multiple runs which differ by their input parameters.  This setting affects only new runs. For example, suppose the job’s concurrency is 4 and there are 4 concurrent active runs. Then setting the concurrency to 3 won’t kill any of the active runs. However, from then on, new runs are skipped unless there are fewer than 3 active runs.  This value cannot exceed 1000\\. Setting this value to 0 causes all new runs to be skipped. The default behavior is to allow only 1 concurrent run. | [optional]
**webhook_notifications** | Option<[**crate::models::JobsWebhookNotifications**](JobsWebhookNotifications.md)> | A collection of system notification IDs to notify when the run begins or completes. The default behavior is to not send any system notifications. | [optional]
**notification_settings** | Option<[**crate::models::JobsJobNotificationSettings**](JobsJobNotificationSettings.md)> | Optional notification settings that are used when sending notifications to each of the `email_notifications` and `webhook_notifications` for this job. | [optional]
**git_source** | Option<[**crate::models::JobsGitSource**](JobsGitSource.md)> | An optional specification for a remote Git repository containing the source code used by tasks. Version-controlled source code is supported by notebook, dbt, Python script, and SQL File tasks.  If `git_source` is set, these tasks retrieve the file from the remote repository by default. However, this behavior can be overridden by setting `source` to `WORKSPACE` on the task.  Note: dbt and SQL File tasks support only version-controlled sources. If dbt or SQL File tasks are used, `git_source` must be defined on the job. | [optional]
**continuous** | Option<[**crate::models::JobsContinuous**](JobsContinuous.md)> | An optional continuous property for this job. The continuous property will ensure that there is always one run executing. Only one of `schedule` and `continuous` can be used. | [optional]
**access_control_list** | Option<[**Vec<crate::models::IamAccessControlRequest>**](IamAccessControlRequest.md)> |  | [optional]
**parameters** | Option<[**Vec<crate::models::JobsJobParameterDefinition>**](JobsJobParameterDefinition.md)> |  | [optional]
**trigger** | Option<[**crate::models::JobsTriggerSettings**](JobsTriggerSettings.md)> | Trigger settings for the job. Can be used to trigger a run when new files arrive in an external location. The default behavior is that the job runs only when triggered by clicking “Run Now” in the Jobs UI or sending an API request to `runNow`. | [optional]
**compute** | Option<[**Vec<crate::models::JobsJobCompute>**](JobsJobCompute.md)> |  | [optional]
**format** | Option<[**crate::models::JobsFormat**](JobsFormat.md)> | Used to tell what is the format of the job. This field is ignored in Create/Update/Reset calls. When using the Jobs API 2.1 this value is always set to `\"MULTI_TASK\"`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


