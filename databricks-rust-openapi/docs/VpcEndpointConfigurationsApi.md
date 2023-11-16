# \VpcEndpointConfigurationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**vpc_endpointscreate**](VpcEndpointConfigurationsApi.md#vpc_endpointscreate) | **POST** /api/2.0/accounts/{account_id}/vpc-endpoints | Create VPC endpoint configuration
[**vpc_endpointsdelete**](VpcEndpointConfigurationsApi.md#vpc_endpointsdelete) | **DELETE** /api/2.0/accounts/{account_id}/vpc-endpoints/{vpc_endpoint_id} | Delete VPC endpoint configuration
[**vpc_endpointsget**](VpcEndpointConfigurationsApi.md#vpc_endpointsget) | **GET** /api/2.0/accounts/{account_id}/vpc-endpoints/{vpc_endpoint_id} | Get a VPC endpoint configuration
[**vpc_endpointslist**](VpcEndpointConfigurationsApi.md#vpc_endpointslist) | **GET** /api/2.0/accounts/{account_id}/vpc-endpoints | Get all VPC endpoint configurations



## vpc_endpointscreate

> crate::models::ProvisioningVpcEndpoint vpc_endpointscreate(provisioning_create_vpc_endpoint_request, x_databricks_gcp_sa_access_token)
Create VPC endpoint configuration

Creates a VPC endpoint configuration, which represents a [VPC endpoint](https://Docsaws.amazon.com/vpc/latest/privatelink/vpc-endpoints.html) object in AWS used to communicate privately with Databricks over [AWS PrivateLink](https://aws.amazon.com/privatelink).  After you create the VPC endpoint configuration, the Databricks [endpoint service](https://Docsaws.amazon.com/vpc/latest/privatelink/privatelink-share-your-services.html) automatically accepts the VPC endpoint.  Before configuring PrivateLink, read the [Databricks article about PrivateLink](https://docs.databricks.com/administration-guide/cloud-configurations/aws/privatelink.html). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**provisioning_create_vpc_endpoint_request** | [**ProvisioningCreateVpcEndpointRequest**](ProvisioningCreateVpcEndpointRequest.md) | Properties of the new VPC endpoint configuration. | [required] |
**x_databricks_gcp_sa_access_token** | Option<[**serde_json::Value**](.md)> | The Google Cloud access token of the caller. For details about this access token, see [Authentication using Open ID Connect (OIDC) tokens](https://Docsgcp.databricks.com/dev-tools/api/latest/authentication-oidc.html). |  |

### Return type

[**crate::models::ProvisioningVpcEndpoint**](ProvisioningVpcEndpoint.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpc_endpointsdelete

> serde_json::Value vpc_endpointsdelete(account_id, vpc_endpoint_id)
Delete VPC endpoint configuration

Deletes a VPC endpoint configuration, which represents an [AWS VPC endpoint](https://Docsaws.amazon.com/vpc/latest/privatelink/concepts.html) that can communicate privately with Databricks over [AWS PrivateLink](https://aws.amazon.com/privatelink).  Before configuring PrivateLink, read the [Databricks article about PrivateLink](https://docs.databricks.com/administration-guide/cloud-configurations/aws/privatelink.html). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**vpc_endpoint_id** | [**serde_json::Value**](.md) | Databricks VPC endpoint ID. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpc_endpointsget

> crate::models::ProvisioningVpcEndpoint vpc_endpointsget(account_id, vpc_endpoint_id)
Get a VPC endpoint configuration

Gets a VPC endpoint configuration, which represents a [VPC endpoint](https://Docsaws.amazon.com/vpc/latest/privatelink/concepts.html) object in AWS used to communicate privately with Databricks over [AWS PrivateLink](https://aws.amazon.com/privatelink). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |
**vpc_endpoint_id** | [**serde_json::Value**](.md) | Databricks VPC endpoint ID. | [required] |

### Return type

[**crate::models::ProvisioningVpcEndpoint**](ProvisioningVpcEndpoint.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vpc_endpointslist

> Vec<crate::models::ProvisioningVpcEndpoint> vpc_endpointslist(account_id)
Get all VPC endpoint configurations

Gets a list of all VPC endpoints for an account, specified by ID.  Before configuring PrivateLink, read the [Databricks article about PrivateLink](https://Docsdatabricks.com/administration-guide/cloud-configurations/aws/privatelink.html). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | Databricks account ID of any type. For non-E2 account types, get your account ID from the [Accounts Console](https://Docsdatabricks.com/administration-guide/account-settings/usage.html). | [required] |

### Return type

[**Vec<crate::models::ProvisioningVpcEndpoint>**](ProvisioningVpcEndpoint.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

