# JobsFileArrivalTriggerConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**min_time_between_triggers_seconds** | Option<**i32**> | If set, the trigger starts a run only after the specified amount of time passed since the last time the trigger fired. The minimum allowed value is 60 seconds  | [optional]
**url** | Option<**String**> | URL to be monitored for file arrivals. The path must point to the root or a subpath of the external location. | [optional]
**wait_after_last_change_seconds** | Option<**i32**> | If set, the trigger starts a run only after no file activity has occurred for the specified amount of time. This makes it possible to wait for a batch of incoming files to arrive before triggering a run. The minimum allowed value is 60 seconds.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


