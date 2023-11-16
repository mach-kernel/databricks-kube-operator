# ProvisioningVpcEndpoint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aws_account_id** | Option<**String**> | The AWS Account in which the VPC endpoint object exists. | [optional]
**vpc_endpoint_name** | Option<**String**> | The human-readable name of the storage configuration. | [optional]
**aws_endpoint_service_id** | Option<**String**> | The ID of the Databricks [endpoint service](https://Docsaws.amazon.com/vpc/latest/privatelink/endpoint-service.html) that this VPC endpoint is connected to. For a list of endpoint service IDs for each supported AWS region, see the [Databricks PrivateLink documentation](https://docs.databricks.com/administration-guide/cloud-configurations/aws/privatelink.html). | [optional]
**use_case** | Option<[**crate::models::ProvisioningEndpointUseCase**](ProvisioningEndpointUseCase.md)> |  | [optional]
**gcp_vpc_endpoint_info** | Option<[**crate::models::ProvisioningGcpVpcEndpointInfo**](ProvisioningGcpVpcEndpointInfo.md)> |  | [optional]
**vpc_endpoint_id** | Option<[**String**](String.md)> | Databricks VPC endpoint ID. This is the Databricks-specific name of the VPC endpoint. Do not confuse this with the `aws_vpc_endpoint_id`, which is the ID within AWS of the VPC endpoint. | [optional]
**aws_vpc_endpoint_id** | Option<**String**> | The ID of the VPC endpoint object in AWS. | [optional]
**region** | Option<**String**> | The AWS region in which this VPC endpoint object exists. | [optional]
**state** | Option<**String**> | The current state (such as `available` or `rejected`) of the VPC endpoint. Derived from AWS. For the full set of values, see [AWS DescribeVpcEndpoint documentation](https://Docsaws.amazon.com/cli/latest/reference/ec2/describe-vpc-endpoints.html). | [optional]
**account_id** | Option<[**String**](String.md)> | The Databricks account ID that hosts the VPC endpoint configuration. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


