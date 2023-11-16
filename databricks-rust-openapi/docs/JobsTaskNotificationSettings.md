# JobsTaskNotificationSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alert_on_last_attempt** | Option<**bool**> | If true, do not send notifications to recipients specified in `on_start` for the retried runs and do not send notifications to recipients specified in `on_failure` until the last retry of the run. | [optional]
**no_alert_for_canceled_runs** | Option<**bool**> | If true, do not send notifications to recipients specified in `on_failure` if the run is canceled. | [optional]
**no_alert_for_skipped_runs** | Option<**bool**> | If true, do not send notifications to recipients specified in `on_failure` if the run is skipped. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


