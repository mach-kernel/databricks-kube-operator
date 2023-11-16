# JobsRunState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**life_cycle_state** | Option<[**crate::models::JobsRunLifeCycleState**](JobsRunLifeCycleState.md)> | A value indicating the run's current lifecycle state. This field is always available in the response. | [optional]
**result_state** | Option<[**crate::models::JobsRunResultState**](JobsRunResultState.md)> | A value indicating the run's result. This field is only available for terminal lifecycle states. | [optional]
**state_message** | Option<**String**> | A descriptive message for the current state. This field is unstructured, and its exact format is subject to change. | [optional]
**user_cancelled_or_timedout** | Option<**bool**> | A value indicating whether a run was canceled manually by a user or by the scheduler because the run timed out. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


