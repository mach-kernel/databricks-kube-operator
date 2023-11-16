# \SecretApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**secretscreate_scope**](SecretApi.md#secretscreate_scope) | **POST** /api/2.0/secrets/scopes/create | Create a new secret scope
[**secretsdelete_acl**](SecretApi.md#secretsdelete_acl) | **POST** /api/2.0/secrets/acls/delete | Delete an ACL
[**secretsdelete_scope**](SecretApi.md#secretsdelete_scope) | **POST** /api/2.0/secrets/scopes/delete | Delete a secret scope
[**secretsdelete_secret**](SecretApi.md#secretsdelete_secret) | **POST** /api/2.0/secrets/delete | Delete a secret
[**secretsget_acl**](SecretApi.md#secretsget_acl) | **GET** /api/2.0/secrets/acls/get | Get secret ACL details
[**secretsget_secret**](SecretApi.md#secretsget_secret) | **GET** /api/2.0/secrets/get | Get a secret
[**secretslist_acls**](SecretApi.md#secretslist_acls) | **GET** /api/2.0/secrets/acls/list | Lists ACLs
[**secretslist_scopes**](SecretApi.md#secretslist_scopes) | **GET** /api/2.0/secrets/scopes/list | List all scopes
[**secretslist_secrets**](SecretApi.md#secretslist_secrets) | **GET** /api/2.0/secrets/list | List secret keys
[**secretsput_acl**](SecretApi.md#secretsput_acl) | **POST** /api/2.0/secrets/acls/put | Create/update an ACL
[**secretsput_secret**](SecretApi.md#secretsput_secret) | **POST** /api/2.0/secrets/put | Add a secret



## secretscreate_scope

> serde_json::Value secretscreate_scope(workspace_create_scope)
Create a new secret scope

The scope name must consist of alphanumeric characters, dashes, underscores, and periods,  and may not exceed 128 characters. The maximum number of scopes in a workspace is 100. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_create_scope** | Option<[**WorkspaceCreateScope**](WorkspaceCreateScope.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secretsdelete_acl

> serde_json::Value secretsdelete_acl(workspace_delete_acl)
Delete an ACL

Deletes the given ACL on the given scope.  Users must have the `MANAGE` permission to invoke this API. Throws `RESOURCE_DOES_NOT_EXIST` if no such secret scope, principal, or ACL exists. Throws `PERMISSION_DENIED` if the user does not have permission to make this API call. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_delete_acl** | Option<[**WorkspaceDeleteAcl**](WorkspaceDeleteAcl.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secretsdelete_scope

> serde_json::Value secretsdelete_scope(workspace_delete_scope)
Delete a secret scope

Deletes a secret scope.   Throws `RESOURCE_DOES_NOT_EXIST` if the scope does not exist. Throws `PERMISSION_DENIED` if the user does not have permission to make this API call. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_delete_scope** | Option<[**WorkspaceDeleteScope**](WorkspaceDeleteScope.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secretsdelete_secret

> serde_json::Value secretsdelete_secret(workspace_delete_secret)
Delete a secret

Deletes the secret stored in this secret scope.  You must have `WRITE` or `MANAGE` permission on the secret scope.  Throws `RESOURCE_DOES_NOT_EXIST` if no such secret scope or secret exists. Throws `PERMISSION_DENIED` if the user does not have permission to make this API call. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_delete_secret** | Option<[**WorkspaceDeleteSecret**](WorkspaceDeleteSecret.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secretsget_acl

> crate::models::WorkspaceAclItem secretsget_acl(scope, principal)
Get secret ACL details

Gets the details about the given ACL, such as the group and permission. Users must have the `MANAGE` permission to invoke this API.  Throws `RESOURCE_DOES_NOT_EXIST` if no such secret scope exists. Throws `PERMISSION_DENIED` if the user does not have permission to make this API call. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | **String** | The name of the scope to fetch ACL information from. | [required] |
**principal** | **String** | The principal to fetch ACL information for. | [required] |

### Return type

[**crate::models::WorkspaceAclItem**](WorkspaceAclItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secretsget_secret

> crate::models::WorkspaceGetSecretResponse secretsget_secret(scope, key)
Get a secret

Gets the bytes representation of a secret value for the specified scope and key.  Users need the READ permission to make this call.  Note that the secret value returned is in bytes. The interpretation of the bytes is determined by the caller in DBUtils and the type the data is decoded into.  Throws ``PERMISSION_DENIED`` if the user does not have permission to make this API call. Throws ``RESOURCE_DOES_NOT_EXIST`` if no such secret or secret scope exists. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | **String** | The name of the scope to fetch secret information from. | [required] |
**key** | **String** | The key to fetch secret for. | [required] |

### Return type

[**crate::models::WorkspaceGetSecretResponse**](WorkspaceGetSecretResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secretslist_acls

> crate::models::WorkspaceListAclsResponse secretslist_acls(scope)
Lists ACLs

List the ACLs for a given secret scope. Users must have the `MANAGE` permission to invoke this API.  Throws `RESOURCE_DOES_NOT_EXIST` if no such secret scope exists. Throws `PERMISSION_DENIED` if the user does not have permission to make this API call. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | **String** | The name of the scope to fetch ACL information from. | [required] |

### Return type

[**crate::models::WorkspaceListAclsResponse**](WorkspaceListAclsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secretslist_scopes

> crate::models::WorkspaceListScopesResponse secretslist_scopes()
List all scopes

Lists all secret scopes available in the workspace.   Throws `PERMISSION_DENIED` if the user does not have permission to make this API call. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WorkspaceListScopesResponse**](WorkspaceListScopesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secretslist_secrets

> crate::models::WorkspaceListSecretsResponse secretslist_secrets(scope)
List secret keys

Lists the secret keys that are stored at this scope.  This is a metadata-only operation; secret data cannot be retrieved using this API.  Users need the READ permission to make this call.  The lastUpdatedTimestamp returned is in milliseconds since epoch. Throws `RESOURCE_DOES_NOT_EXIST` if no such secret scope exists. Throws `PERMISSION_DENIED` if the user does not have permission to make this API call. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | **String** | The name of the scope to list secrets within. | [required] |

### Return type

[**crate::models::WorkspaceListSecretsResponse**](WorkspaceListSecretsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secretsput_acl

> serde_json::Value secretsput_acl(workspace_put_acl)
Create/update an ACL

Creates or overwrites the Access Control List (ACL) associated with the given principal (user or group) on the specified scope point.  In general, a user or group will use the most powerful permission available to them, and permissions are ordered as follows:  * `MANAGE` - Allowed to change ACLs, and read and write to this secret scope. * `WRITE` - Allowed to read and write to this secret scope. * `READ` - Allowed to read this secret scope and list what secrets are available.  Note that in general, secret values can only be read from within a command on a cluster (for example, through a notebook). There is no API to read the actual secret value material outside of a cluster. However, the user's permission will be applied based on who is executing the command, and they must have at least READ permission.  Users must have the `MANAGE` permission to invoke this API.  The principal is a user or group name corresponding to an existing Databricks principal to be granted or revoked access.  Throws `RESOURCE_DOES_NOT_EXIST` if no such secret scope exists. Throws `RESOURCE_ALREADY_EXISTS` if a permission for the principal already exists. Throws `INVALID_PARAMETER_VALUE` if the permission or principal is invalid. Throws `PERMISSION_DENIED` if the user does not have permission to make this API call. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_put_acl** | Option<[**WorkspacePutAcl**](WorkspacePutAcl.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secretsput_secret

> serde_json::Value secretsput_secret(workspace_put_secret)
Add a secret

Inserts a secret under the provided scope with the given name.  If a secret already exists with the same name, this command overwrites the existing secret's value. The server encrypts the secret using the secret scope's encryption settings before storing it.   You must have `WRITE` or `MANAGE` permission on the secret scope. The secret key must consist of alphanumeric characters, dashes, underscores, and periods, and cannot exceed 128 characters.  The maximum allowed secret value size is 128 KB. The maximum number of secrets in a given scope is 1000.  The input fields \"string_value\" or \"bytes_value\" specify the type of the secret, which will determine the value returned when  the secret value is requested. Exactly one must be specified.  Throws `RESOURCE_DOES_NOT_EXIST` if no such secret scope exists. Throws `RESOURCE_LIMIT_EXCEEDED` if maximum number of secrets in scope is exceeded. Throws `INVALID_PARAMETER_VALUE` if the key name or value length is invalid. Throws `PERMISSION_DENIED` if the user does not have permission to make this API call. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_put_secret** | Option<[**WorkspacePutSecret**](WorkspacePutSecret.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

