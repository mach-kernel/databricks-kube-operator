# \ModelRegistryApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**model_registryapprove_transition_request**](ModelRegistryApi.md#model_registryapprove_transition_request) | **POST** /api/2.0/mlflow/transition-requests/approve | Approve transition request
[**model_registrycreate_comment**](ModelRegistryApi.md#model_registrycreate_comment) | **POST** /api/2.0/mlflow/comments/create | Post a comment
[**model_registrycreate_model**](ModelRegistryApi.md#model_registrycreate_model) | **POST** /api/2.0/mlflow/registered-models/create | Create a model
[**model_registrycreate_model_version**](ModelRegistryApi.md#model_registrycreate_model_version) | **POST** /api/2.0/mlflow/model-versions/create | Create a model version
[**model_registrycreate_transition_request**](ModelRegistryApi.md#model_registrycreate_transition_request) | **POST** /api/2.0/mlflow/transition-requests/create | Make a transition request
[**model_registrycreate_webhook**](ModelRegistryApi.md#model_registrycreate_webhook) | **POST** /api/2.0/mlflow/registry-webhooks/create | Create a webhook
[**model_registrydelete_comment**](ModelRegistryApi.md#model_registrydelete_comment) | **DELETE** /api/2.0/mlflow/comments/delete | Delete a comment
[**model_registrydelete_model**](ModelRegistryApi.md#model_registrydelete_model) | **DELETE** /api/2.0/mlflow/registered-models/delete | Delete a model
[**model_registrydelete_model_tag**](ModelRegistryApi.md#model_registrydelete_model_tag) | **DELETE** /api/2.0/mlflow/registered-models/delete-tag | Delete a model tag
[**model_registrydelete_model_version**](ModelRegistryApi.md#model_registrydelete_model_version) | **DELETE** /api/2.0/mlflow/model-versions/delete | Delete a model version.
[**model_registrydelete_model_version_tag**](ModelRegistryApi.md#model_registrydelete_model_version_tag) | **DELETE** /api/2.0/mlflow/model-versions/delete-tag | Delete a model version tag
[**model_registrydelete_transition_request**](ModelRegistryApi.md#model_registrydelete_transition_request) | **DELETE** /api/2.0/mlflow/transition-requests/delete | Delete a transition request
[**model_registrydelete_webhook**](ModelRegistryApi.md#model_registrydelete_webhook) | **DELETE** /api/2.0/mlflow/registry-webhooks/delete | Delete a webhook
[**model_registryget_latest_versions**](ModelRegistryApi.md#model_registryget_latest_versions) | **POST** /api/2.0/mlflow/registered-models/get-latest-versions | Get the latest version
[**model_registryget_model**](ModelRegistryApi.md#model_registryget_model) | **GET** /api/2.0/mlflow/databricks/registered-models/get | Get model
[**model_registryget_model_version**](ModelRegistryApi.md#model_registryget_model_version) | **GET** /api/2.0/mlflow/model-versions/get | Get a model version
[**model_registryget_model_version_download_uri**](ModelRegistryApi.md#model_registryget_model_version_download_uri) | **GET** /api/2.0/mlflow/model-versions/get-download-uri | Get a model version URI
[**model_registryget_registered_model_permission_levels**](ModelRegistryApi.md#model_registryget_registered_model_permission_levels) | **GET** /api/2.0/permissions/registered-models/{registered_model_id}/permissionLevels | Get registered model permission levels
[**model_registryget_registered_model_permissions**](ModelRegistryApi.md#model_registryget_registered_model_permissions) | **GET** /api/2.0/permissions/registered-models/{registered_model_id} | Get registered model permissions
[**model_registrylist_models**](ModelRegistryApi.md#model_registrylist_models) | **GET** /api/2.0/mlflow/registered-models/list | List models
[**model_registrylist_transition_requests**](ModelRegistryApi.md#model_registrylist_transition_requests) | **GET** /api/2.0/mlflow/transition-requests/list | List transition requests
[**model_registrylist_webhooks**](ModelRegistryApi.md#model_registrylist_webhooks) | **GET** /api/2.0/mlflow/registry-webhooks/list | List registry webhooks
[**model_registryreject_transition_request**](ModelRegistryApi.md#model_registryreject_transition_request) | **POST** /api/2.0/mlflow/transition-requests/reject | Reject a transition request
[**model_registryrename_model**](ModelRegistryApi.md#model_registryrename_model) | **POST** /api/2.0/mlflow/registered-models/rename | Rename a model
[**model_registrysearch_model_versions**](ModelRegistryApi.md#model_registrysearch_model_versions) | **GET** /api/2.0/mlflow/model-versions/search | Searches model versions
[**model_registrysearch_models**](ModelRegistryApi.md#model_registrysearch_models) | **GET** /api/2.0/mlflow/registered-models/search | Search models
[**model_registryset_model_tag**](ModelRegistryApi.md#model_registryset_model_tag) | **POST** /api/2.0/mlflow/registered-models/set-tag | Set a tag
[**model_registryset_model_version_tag**](ModelRegistryApi.md#model_registryset_model_version_tag) | **POST** /api/2.0/mlflow/model-versions/set-tag | Set a version tag
[**model_registryset_registered_model_permissions**](ModelRegistryApi.md#model_registryset_registered_model_permissions) | **PUT** /api/2.0/permissions/registered-models/{registered_model_id} | Set registered model permissions
[**model_registrytest_registry_webhook**](ModelRegistryApi.md#model_registrytest_registry_webhook) | **POST** /api/2.0/mlflow/registry-webhooks/test | Test a webhook
[**model_registrytransition_stage**](ModelRegistryApi.md#model_registrytransition_stage) | **POST** /api/2.0/mlflow/databricks/model-versions/transition-stage | Transition a stage
[**model_registryupdate_comment**](ModelRegistryApi.md#model_registryupdate_comment) | **PATCH** /api/2.0/mlflow/comments/update | Update a comment
[**model_registryupdate_model**](ModelRegistryApi.md#model_registryupdate_model) | **PATCH** /api/2.0/mlflow/registered-models/update | Update model
[**model_registryupdate_model_version**](ModelRegistryApi.md#model_registryupdate_model_version) | **PATCH** /api/2.0/mlflow/model-versions/update | Update model version
[**model_registryupdate_registered_model_permissions**](ModelRegistryApi.md#model_registryupdate_registered_model_permissions) | **PATCH** /api/2.0/permissions/registered-models/{registered_model_id} | Update registered model permissions
[**model_registryupdate_webhook**](ModelRegistryApi.md#model_registryupdate_webhook) | **PATCH** /api/2.0/mlflow/registry-webhooks/update | Update a webhook



## model_registryapprove_transition_request

> crate::models::ModelRegistryapproveTransitionRequest200Response model_registryapprove_transition_request(ml_approve_transition_request)
Approve transition request

Approves a model version stage transition request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_approve_transition_request** | [**MlApproveTransitionRequest**](MlApproveTransitionRequest.md) | Details required to identify and approve a model version stage transition request. | [required] |

### Return type

[**crate::models::ModelRegistryapproveTransitionRequest200Response**](ModelRegistryapproveTransitionRequest_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registrycreate_comment

> crate::models::ModelRegistrycreateComment200Response model_registrycreate_comment(ml_create_comment)
Post a comment

Posts a comment on a model version. A comment can be submitted either by a user or programmatically to display relevant information about the model. For example, test results or deployment errors. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_create_comment** | [**MlCreateComment**](MlCreateComment.md) | Details required to create a comment on a model version. | [required] |

### Return type

[**crate::models::ModelRegistrycreateComment200Response**](ModelRegistrycreateComment_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registrycreate_model

> crate::models::MlCreateModelResponse model_registrycreate_model(ml_create_model_request)
Create a model

Creates a new registered model with the name specified in the request body.  Throws `RESOURCE_ALREADY_EXISTS` if a registered model with the given name exists. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_create_model_request** | Option<[**MlCreateModelRequest**](MlCreateModelRequest.md)> |  |  |

### Return type

[**crate::models::MlCreateModelResponse**](MlCreateModelResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registrycreate_model_version

> crate::models::MlCreateModelVersionResponse model_registrycreate_model_version(ml_create_model_version_request)
Create a model version

Creates a model version. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_create_model_version_request** | Option<[**MlCreateModelVersionRequest**](MlCreateModelVersionRequest.md)> |  |  |

### Return type

[**crate::models::MlCreateModelVersionResponse**](MlCreateModelVersionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registrycreate_transition_request

> crate::models::ModelRegistrycreateTransitionRequest200Response model_registrycreate_transition_request(ml_create_transition_request)
Make a transition request

Creates a model version stage transition request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_create_transition_request** | [**MlCreateTransitionRequest**](MlCreateTransitionRequest.md) | Details required to create a model version stage transition request. | [required] |

### Return type

[**crate::models::ModelRegistrycreateTransitionRequest200Response**](ModelRegistrycreateTransitionRequest_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registrycreate_webhook

> crate::models::ModelRegistrycreateWebhook200Response model_registrycreate_webhook(ml_create_registry_webhook)
Create a webhook

**NOTE**: This endpoint is in Public Preview.  Creates a registry webhook. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_create_registry_webhook** | [**MlCreateRegistryWebhook**](MlCreateRegistryWebhook.md) | Details required to create a registry webhook. | [required] |

### Return type

[**crate::models::ModelRegistrycreateWebhook200Response**](ModelRegistrycreateWebhook_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registrydelete_comment

> model_registrydelete_comment(id)
Delete a comment

Deletes a comment on a model version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registrydelete_model

> serde_json::Value model_registrydelete_model(name)
Delete a model

Deletes a registered model. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Registered model unique name identifier. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registrydelete_model_tag

> serde_json::Value model_registrydelete_model_tag(name, key)
Delete a model tag

Deletes the tag for a registered model. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the registered model that the tag was logged under. | [required] |
**key** | **String** | Name of the tag. The name must be an exact match; wild-card deletion is not supported. Maximum size is 250 bytes. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registrydelete_model_version

> serde_json::Value model_registrydelete_model_version(name, version)
Delete a model version.

Deletes a model version.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the registered model | [required] |
**version** | **String** | Model version number | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registrydelete_model_version_tag

> serde_json::Value model_registrydelete_model_version_tag(name, version, key)
Delete a model version tag

Deletes a model version tag. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the registered model that the tag was logged under. | [required] |
**version** | **String** | Model version number that the tag was logged under. | [required] |
**key** | **String** | Name of the tag. The name must be an exact match; wild-card deletion is not supported. Maximum size is 250 bytes. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registrydelete_transition_request

> model_registrydelete_transition_request(name, version, stage, creator, comment)
Delete a transition request

Cancels a model version stage transition request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**serde_json::Value**](.md) | Name of the model. | [required] |
**version** | [**serde_json::Value**](.md) | Version of the model. | [required] |
**stage** | **String** | Target stage of the transition request. Valid values are:  * `None`: The initial stage of a model version.  * `Staging`: Staging or pre-production stage.  * `Production`: Production stage.  * `Archived`: Archived stage. | [required] |
**creator** | **String** | Username of the user who created this request. Of the transition requests matching the specified details, only the one transition created by this user will be deleted. | [required] |
**comment** | Option<[**serde_json::Value**](.md)> | User-provided comment on the action. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registrydelete_webhook

> model_registrydelete_webhook(id)
Delete a webhook

**NOTE:** This endpoint is in Public Preview.  Deletes a registry webhook. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<**String**> | Webhook ID required to delete a registry webhook. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registryget_latest_versions

> crate::models::MlGetLatestVersionsResponse model_registryget_latest_versions(ml_get_latest_versions_request)
Get the latest version

Gets the latest version of a registered model. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_get_latest_versions_request** | Option<[**MlGetLatestVersionsRequest**](MlGetLatestVersionsRequest.md)> |  |  |

### Return type

[**crate::models::MlGetLatestVersionsResponse**](MlGetLatestVersionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registryget_model

> crate::models::ModelRegistrygetModel200Response model_registryget_model(name)
Get model

Get the details of a model. This is a Databricks workspace version of the [MLflow endpoint](https://Wwwmlflow.org/docs/latest/rest-api.html#get-registeredmodel) that also returns the model's Databricks workspace ID and the permission level of the requesting user on the model. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Registered model unique name identifier. | [required] |

### Return type

[**crate::models::ModelRegistrygetModel200Response**](ModelRegistrygetModel_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registryget_model_version

> crate::models::MlGetModelVersionResponse model_registryget_model_version(name, version)
Get a model version

Get a model version. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the registered model | [required] |
**version** | **String** | Model version number | [required] |

### Return type

[**crate::models::MlGetModelVersionResponse**](MlGetModelVersionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registryget_model_version_download_uri

> crate::models::MlGetModelVersionDownloadUriResponse model_registryget_model_version_download_uri(name, version)
Get a model version URI

Gets a URI to download the model version. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the registered model | [required] |
**version** | **String** | Model version number | [required] |

### Return type

[**crate::models::MlGetModelVersionDownloadUriResponse**](MlGetModelVersionDownloadUriResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registryget_registered_model_permission_levels

> crate::models::MlGetRegisteredModelPermissionLevelsResponse model_registryget_registered_model_permission_levels(registered_model_id)
Get registered model permission levels

Gets the permission levels that a user can have on an object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registered_model_id** | [**serde_json::Value**](.md) | The registered model for which to get or manage permissions. | [required] |

### Return type

[**crate::models::MlGetRegisteredModelPermissionLevelsResponse**](MlGetRegisteredModelPermissionLevelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registryget_registered_model_permissions

> crate::models::MlRegisteredModelPermissions model_registryget_registered_model_permissions(registered_model_id)
Get registered model permissions

Gets the permissions of a registered model. Registered models can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registered_model_id** | [**serde_json::Value**](.md) | The registered model for which to get or manage permissions. | [required] |

### Return type

[**crate::models::MlRegisteredModelPermissions**](MlRegisteredModelPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registrylist_models

> crate::models::MlListModelsResponse model_registrylist_models(max_results, page_token)
List models

Lists all available registered models, up to the limit specified in __max_results__. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max_results** | Option<**i32**> | Maximum number of registered models desired. Max threshold is 1000. |  |
**page_token** | Option<**String**> | Pagination token to go to the next page based on a previous query. |  |

### Return type

[**crate::models::MlListModelsResponse**](MlListModelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registrylist_transition_requests

> crate::models::ModelRegistrylistTransitionRequests200Response model_registrylist_transition_requests(name, version)
List transition requests

Gets a list of all open stage transition requests for the model version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**serde_json::Value**](.md) | Name of the model. | [required] |
**version** | [**serde_json::Value**](.md) | Version of the model. | [required] |

### Return type

[**crate::models::ModelRegistrylistTransitionRequests200Response**](ModelRegistrylistTransitionRequests_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registrylist_webhooks

> crate::models::MlListRegistryWebhooks model_registrylist_webhooks(model_name, events, page_token)
List registry webhooks

**NOTE:** This endpoint is in Public Preview.  Lists all registry webhooks. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model_name** | Option<**String**> | If not specified, all webhooks associated with the specified events are listed, regardless of their associated model. |  |
**events** | Option<[**Vec<crate::models::MlRegistryWebhookEvent>**](crate::models::MlRegistryWebhookEvent.md)> | If `events` is specified, any webhook with one or more of the specified trigger events is included in the output. If `events` is not specified, webhooks of all event types are included in the output. |  |
**page_token** | Option<**String**> | Token indicating the page of artifact results to fetch |  |

### Return type

[**crate::models::MlListRegistryWebhooks**](MlListRegistryWebhooks.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registryreject_transition_request

> crate::models::ModelRegistryapproveTransitionRequest200Response model_registryreject_transition_request(ml_reject_transition_request)
Reject a transition request

Rejects a model version stage transition request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_reject_transition_request** | [**MlRejectTransitionRequest**](MlRejectTransitionRequest.md) | Details required to identify and reject a model version stage transition request. | [required] |

### Return type

[**crate::models::ModelRegistryapproveTransitionRequest200Response**](ModelRegistryapproveTransitionRequest_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registryrename_model

> crate::models::MlRenameModelResponse model_registryrename_model(ml_rename_model_request)
Rename a model

Renames a registered model. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_rename_model_request** | Option<[**MlRenameModelRequest**](MlRenameModelRequest.md)> |  |  |

### Return type

[**crate::models::MlRenameModelResponse**](MlRenameModelResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registrysearch_model_versions

> crate::models::MlSearchModelVersionsResponse model_registrysearch_model_versions(filter, max_results, order_by, page_token)
Searches model versions

Searches for specific model versions based on the supplied __filter__. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | String filter condition, like \"name='my-model-name'\". Must be a single boolean condition, with string values wrapped in single quotes. |  |
**max_results** | Option<**i32**> | Maximum number of models desired. Max threshold is 10K. |  |
**order_by** | Option<[**Vec<String>**](String.md)> | List of columns to be ordered by including model name, version, stage with an optional \"DESC\" or \"ASC\" annotation, where \"ASC\" is the default. Tiebreaks are done by latest stage transition timestamp, followed by name ASC, followed by version DESC. |  |
**page_token** | Option<**String**> | Pagination token to go to next page based on previous search query. |  |

### Return type

[**crate::models::MlSearchModelVersionsResponse**](MlSearchModelVersionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registrysearch_models

> crate::models::MlSearchModelsResponse model_registrysearch_models(filter, max_results, order_by, page_token)
Search models

Search for registered models based on the specified __filter__. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | String filter condition, like \"name LIKE 'my-model-name'\". Interpreted in the backend automatically as \"name LIKE '%my-model-name%'\". Single boolean condition, with string values wrapped in single quotes. |  |
**max_results** | Option<**i32**> | Maximum number of models desired. Default is 100. Max threshold is 1000. |  |
**order_by** | Option<[**Vec<String>**](String.md)> | List of columns for ordering search results, which can include model name and last updated timestamp with an optional \"DESC\" or \"ASC\" annotation, where \"ASC\" is the default. Tiebreaks are done by model name ASC. |  |
**page_token** | Option<**String**> | Pagination token to go to the next page based on a previous search query. |  |

### Return type

[**crate::models::MlSearchModelsResponse**](MlSearchModelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registryset_model_tag

> serde_json::Value model_registryset_model_tag(ml_set_model_tag_request)
Set a tag

Sets a tag on a registered model. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_set_model_tag_request** | Option<[**MlSetModelTagRequest**](MlSetModelTagRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registryset_model_version_tag

> serde_json::Value model_registryset_model_version_tag(ml_set_model_version_tag_request)
Set a version tag

Sets a model version tag. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_set_model_version_tag_request** | Option<[**MlSetModelVersionTagRequest**](MlSetModelVersionTagRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registryset_registered_model_permissions

> crate::models::MlRegisteredModelPermissions model_registryset_registered_model_permissions(registered_model_id, ml_registered_model_permissions_request)
Set registered model permissions

Sets permissions on a registered model. Registered models can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registered_model_id** | [**serde_json::Value**](.md) | The registered model for which to get or manage permissions. | [required] |
**ml_registered_model_permissions_request** | Option<[**MlRegisteredModelPermissionsRequest**](MlRegisteredModelPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::MlRegisteredModelPermissions**](MlRegisteredModelPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registrytest_registry_webhook

> crate::models::ModelRegistrytestRegistryWebhook200Response model_registrytest_registry_webhook(ml_test_registry_webhook_request)
Test a webhook

**NOTE:** This endpoint is in Public Preview.  Tests a registry webhook. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_test_registry_webhook_request** | [**MlTestRegistryWebhookRequest**](MlTestRegistryWebhookRequest.md) | Details required to test a registry webhook. | [required] |

### Return type

[**crate::models::ModelRegistrytestRegistryWebhook200Response**](ModelRegistrytestRegistryWebhook_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registrytransition_stage

> crate::models::ModelRegistrytransitionStage200Response model_registrytransition_stage(ml_transition_model_version_stage_databricks)
Transition a stage

Transition a model version's stage. This is a Databricks workspace version of the [MLflow endpoint](https://Wwwmlflow.org/docs/latest/rest-api.html#transition-modelversion-stage) that also accepts a comment associated with the transition to be recorded.\", 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_transition_model_version_stage_databricks** | [**MlTransitionModelVersionStageDatabricks**](MlTransitionModelVersionStageDatabricks.md) | Details required to transition a model version's stage. | [required] |

### Return type

[**crate::models::ModelRegistrytransitionStage200Response**](ModelRegistrytransitionStage_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registryupdate_comment

> crate::models::ModelRegistrycreateComment200Response model_registryupdate_comment(ml_update_comment)
Update a comment

Post an edit to a comment on a model version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_update_comment** | [**MlUpdateComment**](MlUpdateComment.md) | Details required to edit a comment on a model version. | [required] |

### Return type

[**crate::models::ModelRegistrycreateComment200Response**](ModelRegistrycreateComment_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registryupdate_model

> serde_json::Value model_registryupdate_model(ml_update_model_request)
Update model

Updates a registered model. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_update_model_request** | Option<[**MlUpdateModelRequest**](MlUpdateModelRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registryupdate_model_version

> serde_json::Value model_registryupdate_model_version(ml_update_model_version_request)
Update model version

Updates the model version. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_update_model_version_request** | Option<[**MlUpdateModelVersionRequest**](MlUpdateModelVersionRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registryupdate_registered_model_permissions

> crate::models::MlRegisteredModelPermissions model_registryupdate_registered_model_permissions(registered_model_id, ml_registered_model_permissions_request)
Update registered model permissions

Updates the permissions on a registered model. Registered models can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**registered_model_id** | [**serde_json::Value**](.md) | The registered model for which to get or manage permissions. | [required] |
**ml_registered_model_permissions_request** | Option<[**MlRegisteredModelPermissionsRequest**](MlRegisteredModelPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::MlRegisteredModelPermissions**](MlRegisteredModelPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## model_registryupdate_webhook

> serde_json::Value model_registryupdate_webhook(ml_update_registry_webhook)
Update a webhook

**NOTE:** This endpoint is in Public Preview.  Updates a registry webhook. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_update_registry_webhook** | [**MlUpdateRegistryWebhook**](MlUpdateRegistryWebhook.md) | Details required to update a registry webhook. Only the fields that need to be updated should be specified, and both `http_url_spec` and `job_spec` should not be specified in the same request. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

