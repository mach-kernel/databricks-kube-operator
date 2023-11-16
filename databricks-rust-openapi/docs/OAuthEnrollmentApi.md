# \OAuthEnrollmentApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**o_auth_enrollmentcreate**](OAuthEnrollmentApi.md#o_auth_enrollmentcreate) | **POST** /api/2.0/accounts/{account_id}/oauth2/enrollment | Create OAuth Enrollment request
[**o_auth_enrollmentget**](OAuthEnrollmentApi.md#o_auth_enrollmentget) | **GET** /api/2.0/accounts/{account_id}/oauth2/enrollment | Get OAuth enrollment status



## o_auth_enrollmentcreate

> o_auth_enrollmentcreate(account_id, oauth2_create_o_auth_enrollment)
Create OAuth Enrollment request

Create an OAuth Enrollment request to enroll OAuth for this account and optionally enable the OAuth integration for all the partner applications in the account.  The parter applications are:   - Power BI   - Tableau Desktop   - Databricks CLI  The enrollment is executed asynchronously, so the API will return 204 immediately. The  actual enrollment take a few minutes, you can check the status via API :method:OAuthEnrollment/get. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | The account ID. | [required] |
**oauth2_create_o_auth_enrollment** | Option<[**Oauth2CreateOAuthEnrollment**](Oauth2CreateOAuthEnrollment.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## o_auth_enrollmentget

> crate::models::Oauth2OAuthEnrollmentStatus o_auth_enrollmentget(account_id)
Get OAuth enrollment status

Gets the OAuth enrollment status for this Account.  You can only add/use the OAuth published/custom application integrations when OAuth enrollment status is enabled. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | [**serde_json::Value**](.md) | The account ID. | [required] |

### Return type

[**crate::models::Oauth2OAuthEnrollmentStatus**](Oauth2OAuthEnrollmentStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

