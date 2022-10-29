# TerminationParameter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**username** | Option<**String**> | The username of the user who terminated the cluster. | [optional]
**aws_api_error_code** | Option<**String**> | The AWS provided error code describing why cluster nodes could not be provisioned. For example, `InstanceLimitExceeded` indicates that the limit of EC2 instances for a specific instance type has been exceeded. For reference, see: <https://docs.aws.amazon.com/AWSEC2/latest/APIReference/query-api-troubleshooting.html>. | [optional]
**aws_instance_state_reason** | Option<**String**> | The AWS provided state reason describing why the driver node was terminated. For example, `Client.VolumeLimitExceeded` indicates that the limit of EBS volumes or total EBS volume storage has been exceeded. For reference, see <https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_StateReason.html>. | [optional]
**aws_spot_request_status** | Option<**String**> | Describes why a spot request could not be fulfilled. For example, `price-too-low` indicates that the max price was lower than the current spot price. For reference, see: <https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/spot-bid-status.html#spot-instance-bid-status-understand>. | [optional]
**aws_spot_request_fault_code** | Option<**String**> | Provides additional details when a spot request fails. For example `InsufficientFreeAddressesInSubnet` indicates the subnet does not have free IP addresses to accommodate the new instance. For reference, see <https://docs.aws.amazon.com/cli/latest/reference/ec2/describe-spot-instance-requests.html>. | [optional]
**aws_impaired_status_details** | Option<**String**> | The AWS provided status check which failed and induced a node loss. This status may correspond to a failed instance or system check. For reference, see <https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/monitoring-system-instance-status-check.html>. | [optional]
**aws_instance_status_event** | Option<**String**> | The AWS provided scheduled event (for example reboot) which induced a node loss. For reference, see <https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/monitoring-instances-status-check_sched.html>. | [optional]
**aws_error_message** | Option<**String**> | Human-readable context of various failures from AWS. This field is unstructured, and its exact format is subject to change. | [optional]
**databricks_error_message** | Option<**String**> | Additional context that may explain the reason for cluster termination. This field is unstructured, and its exact format is subject to change. | [optional]
**inactivity_duration_min** | Option<**String**> | An idle cluster was shut down after being inactive for this duration. | [optional]
**instance_id** | Option<**String**> | The ID of the instance that was hosting the Spark driver. | [optional]
**instance_pool_id** | Option<**String**> | The ID of the instance pool the cluster is using. | [optional]
**instance_pool_error_code** | Option<**String**> | The [error code](https://docs.databricks.com/dev-tools/api/latest/clusters.html#clusterterminationreasonpoolclusterterminationcode) for cluster failures specific to a pool. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


