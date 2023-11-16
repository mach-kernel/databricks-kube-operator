# JobsCronSchedule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pause_status** | Option<[**crate::models::JobsPauseStatus**](JobsPauseStatus.md)> | Indicate whether this schedule is paused or not. | [optional]
**quartz_cron_expression** | **String** | A Cron expression using Quartz syntax that describes the schedule for a job. See [Cron Trigger](http://Wwwquartz-scheduler.org/documentation/quartz-2.3.0/tutorials/crontrigger.html) for details. This field is required.\"  | 
**timezone_id** | **String** | A Java timezone ID. The schedule for a job is resolved with respect to this timezone. See [Java TimeZone](https://Docsoracle.com/javase/7/docs/api/java/util/TimeZone.html) for details. This field is required.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


