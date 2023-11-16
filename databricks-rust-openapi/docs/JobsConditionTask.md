# JobsConditionTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**left** | Option<**String**> | The left operand of the condition task. Can be either a string value or a job state or parameter reference. | [optional]
**op** | Option<**String**> | * `EQUAL_TO`, `NOT_EQUAL` operators perform string comparison of their operands. This means that `“12.0” == “12”` will evaluate to `false`. * `GREATER_THAN`, `GREATER_THAN_OR_EQUAL`, `LESS_THAN`, `LESS_THAN_OR_EQUAL` operators perform numeric comparison of their operands. `“12.0” >= “12”` will evaluate to `true`, `“10.0” >= “12”` will evaluate to `false`.  The boolean comparison to task values can be implemented with operators `EQUAL_TO`, `NOT_EQUAL`. If a task value was set to a boolean value, it will be serialized to `“true”` or `“false”` for the comparison.  | [optional]
**right** | Option<**String**> | The right operand of the condition task. Can be either a string value or a job state or parameter reference. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


