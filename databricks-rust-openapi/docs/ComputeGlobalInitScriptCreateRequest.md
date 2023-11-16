# ComputeGlobalInitScriptCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | Specifies whether the script is enabled. The script runs only if enabled. | [optional][default to false]
**name** | **String** | The name of the script | 
**position** | Option<**i32**> | The position of a global init script, where 0 represents the first script to run, 1 is the second script to run, in ascending order.  If you omit the numeric position for a new global init script, it defaults to last position. It will run after all current scripts. Setting any value greater than the position of the last script is equivalent to the last position. Example: Take three existing scripts with positions 0, 1, and 2. Any position of (3) or greater puts the script in the last position. If an explicit position value conflicts with an existing script value, your request succeeds, but the original script at that position and all later scripts have their positions incremented by 1. | [optional]
**script** | **String** | The Base64-encoded content of the script. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


