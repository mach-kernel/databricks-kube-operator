# ComputeSparkNode

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**host_private_ip** | Option<**String**> | The private IP address of the host instance. | [optional]
**instance_id** | Option<**String**> | Globally unique identifier for the host instance from the cloud provider. | [optional]
**node_aws_attributes** | Option<[**crate::models::ComputeSparkNodeAwsAttributes**](ComputeSparkNodeAwsAttributes.md)> | Attributes specific to AWS for a Spark node. | [optional]
**node_id** | Option<**String**> | Globally unique identifier for this node. | [optional]
**private_ip** | Option<**String**> | Private IP address (typically a 10.Xx.x address) of the Spark node. Note that this is different from the private IP address of the host instance. | [optional]
**public_dns** | Option<**String**> | Public DNS address of this node. This address can be used to access the Spark JDBC server on the driver node. To communicate with the JDBC server, traffic must be manually authorized by adding security group rules to the \"worker-unmanaged\" security group via the AWS console.  Actually it's the public DNS address of the host instance. | [optional]
**start_timestamp** | Option<**i64**> | The timestamp (in millisecond) when the Spark node is launched.  The start_timestamp is set right before the container is being launched. The timestamp when the container is placed on the ResourceManager, before its launch and setup by the NodeDaemon. This timestamp is the same as the creation timestamp in the database. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


