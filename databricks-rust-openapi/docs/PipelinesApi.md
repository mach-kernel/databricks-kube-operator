# \PipelinesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**pipelinescreate**](PipelinesApi.md#pipelinescreate) | **POST** /api/2.0/pipelines | Create a pipeline
[**pipelinesdelete**](PipelinesApi.md#pipelinesdelete) | **DELETE** /api/2.0/pipelines/{pipeline_id} | Delete a pipeline
[**pipelinesget**](PipelinesApi.md#pipelinesget) | **GET** /api/2.0/pipelines/{pipeline_id} | Get a pipeline
[**pipelinesget_pipeline_permission_levels**](PipelinesApi.md#pipelinesget_pipeline_permission_levels) | **GET** /api/2.0/permissions/pipelines/{pipeline_id}/permissionLevels | Get pipeline permission levels
[**pipelinesget_pipeline_permissions**](PipelinesApi.md#pipelinesget_pipeline_permissions) | **GET** /api/2.0/permissions/pipelines/{pipeline_id} | Get pipeline permissions
[**pipelinesget_update**](PipelinesApi.md#pipelinesget_update) | **GET** /api/2.0/pipelines/{pipeline_id}/updates/{update_id} | Get a pipeline update
[**pipelineslist_pipeline_events**](PipelinesApi.md#pipelineslist_pipeline_events) | **GET** /api/2.0/pipelines/{pipeline_id}/events | List pipeline events
[**pipelineslist_pipelines**](PipelinesApi.md#pipelineslist_pipelines) | **GET** /api/2.0/pipelines | List pipelines
[**pipelineslist_updates**](PipelinesApi.md#pipelineslist_updates) | **GET** /api/2.0/pipelines/{pipeline_id}/updates | List pipeline updates
[**pipelinesreset**](PipelinesApi.md#pipelinesreset) | **POST** /api/2.0/pipelines/{pipeline_id}/reset | Reset a pipeline
[**pipelinesset_pipeline_permissions**](PipelinesApi.md#pipelinesset_pipeline_permissions) | **PUT** /api/2.0/permissions/pipelines/{pipeline_id} | Set pipeline permissions
[**pipelinesstart_update**](PipelinesApi.md#pipelinesstart_update) | **POST** /api/2.0/pipelines/{pipeline_id}/updates | Queue a pipeline update
[**pipelinesstop**](PipelinesApi.md#pipelinesstop) | **POST** /api/2.0/pipelines/{pipeline_id}/stop | Stop a pipeline
[**pipelinesupdate**](PipelinesApi.md#pipelinesupdate) | **PUT** /api/2.0/pipelines/{pipeline_id} | Edit a pipeline
[**pipelinesupdate_pipeline_permissions**](PipelinesApi.md#pipelinesupdate_pipeline_permissions) | **PATCH** /api/2.0/permissions/pipelines/{pipeline_id} | Update pipeline permissions



## pipelinescreate

> crate::models::PipelinesCreatePipelineResponse pipelinescreate(pipelines_create_pipeline)
Create a pipeline

Creates a new data processing pipeline based on the requested configuration. If successful, this method returns the ID of the new pipeline. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipelines_create_pipeline** | Option<[**PipelinesCreatePipeline**](PipelinesCreatePipeline.md)> |  |  |

### Return type

[**crate::models::PipelinesCreatePipelineResponse**](PipelinesCreatePipelineResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pipelinesdelete

> serde_json::Value pipelinesdelete(pipeline_id)
Delete a pipeline

Deletes a pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pipelinesget

> crate::models::PipelinesGetPipelineResponse pipelinesget(pipeline_id)
Get a pipeline



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **String** |  | [required] |

### Return type

[**crate::models::PipelinesGetPipelineResponse**](PipelinesGetPipelineResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pipelinesget_pipeline_permission_levels

> crate::models::PipelinesGetPipelinePermissionLevelsResponse pipelinesget_pipeline_permission_levels(pipeline_id)
Get pipeline permission levels

Gets the permission levels that a user can have on an object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **String** | The pipeline for which to get or manage permissions. | [required] |

### Return type

[**crate::models::PipelinesGetPipelinePermissionLevelsResponse**](PipelinesGetPipelinePermissionLevelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pipelinesget_pipeline_permissions

> crate::models::PipelinesPipelinePermissions pipelinesget_pipeline_permissions(pipeline_id)
Get pipeline permissions

Gets the permissions of a pipeline. Pipelines can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | [**serde_json::Value**](.md) | The pipeline for which to get or manage permissions. | [required] |

### Return type

[**crate::models::PipelinesPipelinePermissions**](PipelinesPipelinePermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pipelinesget_update

> crate::models::PipelinesGetUpdateResponse pipelinesget_update(pipeline_id, update_id)
Get a pipeline update

Gets an update from an active pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **String** | The ID of the pipeline. | [required] |
**update_id** | **String** | The ID of the update. | [required] |

### Return type

[**crate::models::PipelinesGetUpdateResponse**](PipelinesGetUpdateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pipelineslist_pipeline_events

> crate::models::PipelinesListPipelineEventsResponse pipelineslist_pipeline_events(pipeline_id, max_results, order_by, filter, page_token)
List pipeline events

Retrieves events for a pipeline. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **String** |  | [required] |
**max_results** | Option<**i32**> | Max number of entries to return in a single page. The system may return  fewer than max_results events in a response, even if there are more events  available.  |  |
**order_by** | Option<[**Vec<String>**](String.md)> | A string indicating a sort order by timestamp for the results, for example, [\"timestamp asc\"]. The sort order can be ascending or descending. By default, events are returned  in descending order by timestamp.  |  |
**filter** | Option<**String**> | Criteria to select a subset of results, expressed using a SQL-like syntax.  The supported filters are: 1. level='INFO' (or WARN or ERROR) 2. level in ('INFO', 'WARN') 3. id='[event-id]' 4. timestamp > 'TIMESTAMP' (or >=,<,<=,=)  Composite expressions are supported, for example: level in ('ERROR', 'WARN')  AND timestamp> '2021-07-22T06:37:33.083Z'  |  |
**page_token** | Option<**String**> | Page token returned by previous call. This field is mutually exclusive with all fields in this request except max_results. An error is returned if any fields other than max_results are set when this field is set.  |  |

### Return type

[**crate::models::PipelinesListPipelineEventsResponse**](PipelinesListPipelineEventsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pipelineslist_pipelines

> crate::models::PipelinesListPipelinesResponse pipelineslist_pipelines(max_results, page_token, order_by, filter)
List pipelines

Lists pipelines defined in the Delta Live Tables system. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max_results** | Option<**i32**> | The maximum number of entries to return in a single page. The system may return fewer than max_results events in a response, even if there are more events available. This field is optional. The default value is 25. The maximum value is 100. An error is returned if the value of max_results is greater than 100.  |  |
**page_token** | Option<**String**> | Page token returned by previous call |  |
**order_by** | Option<[**Vec<String>**](String.md)> | A list of strings specifying the order of results. Supported order_by fields are id and name. The default is id asc. This field is optional.  |  |
**filter** | Option<**String**> | Select a subset of results based on the specified criteria. The supported filters are:  * `notebook='<path>'` to select pipelines that reference the provided notebook path. * `name LIKE '[pattern]'` to select pipelines with a name that matches pattern.   Wildcards are supported, for example: `name LIKE '%shopping%'`  Composite filters are not supported. This field is optional.  |  |

### Return type

[**crate::models::PipelinesListPipelinesResponse**](PipelinesListPipelinesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pipelineslist_updates

> crate::models::PipelinesListUpdatesResponse pipelineslist_updates(pipeline_id, page_token, max_results, until_update_id)
List pipeline updates

List updates for an active pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **String** | The pipeline to return updates for. | [required] |
**page_token** | Option<**String**> | Page token returned by previous call |  |
**max_results** | Option<**i32**> | Max number of entries to return in a single page. |  |
**until_update_id** | Option<**String**> | If present, returns updates until and including this update_id. |  |

### Return type

[**crate::models::PipelinesListUpdatesResponse**](PipelinesListUpdatesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pipelinesreset

> serde_json::Value pipelinesreset(pipeline_id)
Reset a pipeline

Resets a pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pipelinesset_pipeline_permissions

> crate::models::PipelinesPipelinePermissions pipelinesset_pipeline_permissions(pipeline_id, pipelines_pipeline_permissions_request)
Set pipeline permissions

Sets permissions on a pipeline. Pipelines can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | [**serde_json::Value**](.md) | The pipeline for which to get or manage permissions. | [required] |
**pipelines_pipeline_permissions_request** | Option<[**PipelinesPipelinePermissionsRequest**](PipelinesPipelinePermissionsRequest.md)> |  |  |

### Return type

[**crate::models::PipelinesPipelinePermissions**](PipelinesPipelinePermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pipelinesstart_update

> crate::models::PipelinesStartUpdateResponse pipelinesstart_update(pipeline_id, pipelines_start_update)
Queue a pipeline update

Starts or queues a pipeline update.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **String** |  | [required] |
**pipelines_start_update** | Option<[**PipelinesStartUpdate**](PipelinesStartUpdate.md)> |  |  |

### Return type

[**crate::models::PipelinesStartUpdateResponse**](PipelinesStartUpdateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pipelinesstop

> serde_json::Value pipelinesstop(pipeline_id)
Stop a pipeline

Stops a pipeline.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pipelinesupdate

> serde_json::Value pipelinesupdate(pipeline_id, pipelines_edit_pipeline)
Edit a pipeline

Updates a pipeline with the supplied configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | **String** |  | [required] |
**pipelines_edit_pipeline** | Option<[**PipelinesEditPipeline**](PipelinesEditPipeline.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pipelinesupdate_pipeline_permissions

> crate::models::PipelinesPipelinePermissions pipelinesupdate_pipeline_permissions(pipeline_id, pipelines_pipeline_permissions_request)
Update pipeline permissions

Updates the permissions on a pipeline. Pipelines can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pipeline_id** | [**serde_json::Value**](.md) | The pipeline for which to get or manage permissions. | [required] |
**pipelines_pipeline_permissions_request** | Option<[**PipelinesPipelinePermissionsRequest**](PipelinesPipelinePermissionsRequest.md)> |  |  |

### Return type

[**crate::models::PipelinesPipelinePermissions**](PipelinesPipelinePermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

