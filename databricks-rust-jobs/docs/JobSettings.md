# JobSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | An optional name for the job. | [optional][default to Untitled]
**tags** | Option<[**serde_json::Value**](.md)> | A map of tags associated with the job. These are forwarded to the cluster as cluster tags for jobs clusters, and are subject to the same limitations as cluster tags. A maximum of 25 tags can be added to the job. | [optional][default to {}]
**tasks** | Option<[**Vec<crate::models::JobTaskSettings>**](JobTaskSettings.md)> | A list of task specifications to be executed by this job. | [optional]
**job_clusters** | Option<[**Vec<crate::models::JobCluster>**](JobCluster.md)> | A list of job cluster specifications that can be shared and reused by tasks of this job. Libraries cannot be declared in a shared job cluster. You must declare dependent libraries in task settings. | [optional]
**email_notifications** | Option<[**crate::models::JobEmailNotifications**](JobEmailNotifications.md)> |  | [optional]
**timeout_seconds** | Option<**i32**> | An optional timeout applied to each run of this job. The default behavior is to have no timeout. | [optional]
**schedule** | Option<[**crate::models::CronSchedule**](CronSchedule.md)> |  | [optional]
**max_concurrent_runs** | Option<**i32**> | An optional maximum allowed number of concurrent runs of the job.  Set this value if you want to be able to execute multiple runs of the same job concurrently. This is useful for example if you trigger your job on a frequent schedule and want to allow consecutive runs to overlap with each other, or if you want to trigger multiple runs which differ by their input parameters.  This setting affects only new runs. For example, suppose the job’s concurrency is 4 and there are 4 concurrent active runs. Then setting the concurrency to 3 won’t kill any of the active runs. However, from then on, new runs are skipped unless there are fewer than 3 active runs.  This value cannot exceed 1000\\. Setting this value to 0 causes all new runs to be skipped. The default behavior is to allow only 1 concurrent run. | [optional]
**git_source** | Option<[**crate::models::GitSource**](GitSource.md)> |  | [optional]
**format** | Option<**String**> | Used to tell what is the format of the job. This field is ignored in Create/Update/Reset calls. When using the Jobs API 2.1 this value is always set to `\"MULTI_TASK\"`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


