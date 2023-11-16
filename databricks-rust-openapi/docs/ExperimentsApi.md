# \ExperimentsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**experimentscreate_experiment**](ExperimentsApi.md#experimentscreate_experiment) | **POST** /api/2.0/mlflow/experiments/create | Create experiment
[**experimentscreate_run**](ExperimentsApi.md#experimentscreate_run) | **POST** /api/2.0/mlflow/runs/create | Create a run
[**experimentsdelete_experiment**](ExperimentsApi.md#experimentsdelete_experiment) | **POST** /api/2.0/mlflow/experiments/delete | Delete an experiment
[**experimentsdelete_run**](ExperimentsApi.md#experimentsdelete_run) | **POST** /api/2.0/mlflow/runs/delete | Delete a run
[**experimentsdelete_runs**](ExperimentsApi.md#experimentsdelete_runs) | **POST** /api/2.0/mlflow/databricks/runs/delete-runs | Delete runs by creation time
[**experimentsdelete_tag**](ExperimentsApi.md#experimentsdelete_tag) | **POST** /api/2.0/mlflow/runs/delete-tag | Delete a tag
[**experimentsget_by_name**](ExperimentsApi.md#experimentsget_by_name) | **GET** /api/2.0/mlflow/experiments/get-by-name | Get metadata
[**experimentsget_experiment**](ExperimentsApi.md#experimentsget_experiment) | **GET** /api/2.0/mlflow/experiments/get | Get an experiment
[**experimentsget_experiment_permission_levels**](ExperimentsApi.md#experimentsget_experiment_permission_levels) | **GET** /api/2.0/permissions/experiments/{experiment_id}/permissionLevels | Get experiment permission levels
[**experimentsget_experiment_permissions**](ExperimentsApi.md#experimentsget_experiment_permissions) | **GET** /api/2.0/permissions/experiments/{experiment_id} | Get experiment permissions
[**experimentsget_history**](ExperimentsApi.md#experimentsget_history) | **GET** /api/2.0/mlflow/metrics/get-history | Get history of a given metric within a run
[**experimentsget_run**](ExperimentsApi.md#experimentsget_run) | **GET** /api/2.0/mlflow/runs/get | Get a run
[**experimentslist_artifacts**](ExperimentsApi.md#experimentslist_artifacts) | **GET** /api/2.0/mlflow/artifacts/list | Get all artifacts
[**experimentslist_experiments**](ExperimentsApi.md#experimentslist_experiments) | **GET** /api/2.0/mlflow/experiments/list | List experiments
[**experimentslog_batch**](ExperimentsApi.md#experimentslog_batch) | **POST** /api/2.0/mlflow/runs/log-batch | Log a batch
[**experimentslog_inputs**](ExperimentsApi.md#experimentslog_inputs) | **POST** /api/2.0/mlflow/runs/log-inputs | Log inputs to a run
[**experimentslog_metric**](ExperimentsApi.md#experimentslog_metric) | **POST** /api/2.0/mlflow/runs/log-metric | Log a metric
[**experimentslog_model**](ExperimentsApi.md#experimentslog_model) | **POST** /api/2.0/mlflow/runs/log-model | Log a model
[**experimentslog_param**](ExperimentsApi.md#experimentslog_param) | **POST** /api/2.0/mlflow/runs/log-parameter | Log a param
[**experimentsrestore_experiment**](ExperimentsApi.md#experimentsrestore_experiment) | **POST** /api/2.0/mlflow/experiments/restore | Restores an experiment
[**experimentsrestore_run**](ExperimentsApi.md#experimentsrestore_run) | **POST** /api/2.0/mlflow/runs/restore | Restore a run
[**experimentsrestore_runs**](ExperimentsApi.md#experimentsrestore_runs) | **POST** /api/2.0/mlflow/databricks/runs/restore-runs | Restore runs by deletion time
[**experimentssearch_experiments**](ExperimentsApi.md#experimentssearch_experiments) | **POST** /api/2.0/mlflow/experiments/search | Search experiments
[**experimentssearch_runs**](ExperimentsApi.md#experimentssearch_runs) | **POST** /api/2.0/mlflow/runs/search | Search for runs
[**experimentsset_experiment_permissions**](ExperimentsApi.md#experimentsset_experiment_permissions) | **PUT** /api/2.0/permissions/experiments/{experiment_id} | Set experiment permissions
[**experimentsset_experiment_tag**](ExperimentsApi.md#experimentsset_experiment_tag) | **POST** /api/2.0/mlflow/experiments/set-experiment-tag | Set a tag
[**experimentsset_tag**](ExperimentsApi.md#experimentsset_tag) | **POST** /api/2.0/mlflow/runs/set-tag | Set a tag
[**experimentsupdate_experiment**](ExperimentsApi.md#experimentsupdate_experiment) | **POST** /api/2.0/mlflow/experiments/update | Update an experiment
[**experimentsupdate_experiment_permissions**](ExperimentsApi.md#experimentsupdate_experiment_permissions) | **PATCH** /api/2.0/permissions/experiments/{experiment_id} | Update experiment permissions
[**experimentsupdate_run**](ExperimentsApi.md#experimentsupdate_run) | **POST** /api/2.0/mlflow/runs/update | Update a run



## experimentscreate_experiment

> crate::models::MlCreateExperimentResponse experimentscreate_experiment(ml_create_experiment)
Create experiment

Creates an experiment with a name. Returns the ID of the newly created experiment. Validates that another experiment with the same name does not already exist and fails  if another experiment with the same name already exists.  Throws `RESOURCE_ALREADY_EXISTS` if a experiment with the given name exists. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_create_experiment** | Option<[**MlCreateExperiment**](MlCreateExperiment.md)> |  |  |

### Return type

[**crate::models::MlCreateExperimentResponse**](MlCreateExperimentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentscreate_run

> crate::models::MlCreateRunResponse experimentscreate_run(ml_create_run)
Create a run

Creates a new run within an experiment.  A run is usually a single execution of a machine learning or data ETL pipeline.  MLflow uses runs to track the `mlflowParam`, `mlflowMetric` and `mlflowRunTag` associated with a single execution. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_create_run** | Option<[**MlCreateRun**](MlCreateRun.md)> |  |  |

### Return type

[**crate::models::MlCreateRunResponse**](MlCreateRunResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentsdelete_experiment

> serde_json::Value experimentsdelete_experiment(ml_delete_experiment)
Delete an experiment

Marks an experiment and associated metadata, runs, metrics, params, and tags for deletion. If the experiment uses FileStore, artifacts associated with experiment are also deleted. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_delete_experiment** | Option<[**MlDeleteExperiment**](MlDeleteExperiment.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentsdelete_run

> serde_json::Value experimentsdelete_run(ml_delete_run)
Delete a run

Marks a run for deletion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_delete_run** | Option<[**MlDeleteRun**](MlDeleteRun.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentsdelete_runs

> crate::models::MlDeleteRunsResponse experimentsdelete_runs(ml_delete_runs)
Delete runs by creation time

Bulk delete runs in an experiment that were created prior to or at the specified timestamp. Deletes at most max_runs per request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_delete_runs** | Option<[**MlDeleteRuns**](MlDeleteRuns.md)> |  |  |

### Return type

[**crate::models::MlDeleteRunsResponse**](MlDeleteRunsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentsdelete_tag

> serde_json::Value experimentsdelete_tag(ml_delete_tag)
Delete a tag

Deletes a tag on a run. Tags are run metadata that can be updated during a run and after a run completes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_delete_tag** | Option<[**MlDeleteTag**](MlDeleteTag.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentsget_by_name

> crate::models::MlGetExperimentByNameResponse experimentsget_by_name(experiment_name)
Get metadata

Gets metadata for an experiment.  This endpoint will return deleted experiments, but prefers the active experiment if an active and deleted experiment  share the same name. If multiple deleted experiments share the same name, the API will return one of them.  Throws `RESOURCE_DOES_NOT_EXIST` if no experiment with the specified name exists. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experiment_name** | **String** | Name of the associated experiment. | [required] |

### Return type

[**crate::models::MlGetExperimentByNameResponse**](MlGetExperimentByNameResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentsget_experiment

> crate::models::MlExperiment experimentsget_experiment(experiment_id)
Get an experiment

Gets metadata for an experiment. This method works on deleted experiments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experiment_id** | **String** | ID of the associated experiment. | [required] |

### Return type

[**crate::models::MlExperiment**](MlExperiment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentsget_experiment_permission_levels

> crate::models::MlGetExperimentPermissionLevelsResponse experimentsget_experiment_permission_levels(experiment_id)
Get experiment permission levels

Gets the permission levels that a user can have on an object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experiment_id** | **String** | The experiment for which to get or manage permissions. | [required] |

### Return type

[**crate::models::MlGetExperimentPermissionLevelsResponse**](MlGetExperimentPermissionLevelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentsget_experiment_permissions

> crate::models::MlExperimentPermissions experimentsget_experiment_permissions(experiment_id)
Get experiment permissions

Gets the permissions of an experiment. Experiments can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experiment_id** | [**serde_json::Value**](.md) | The experiment for which to get or manage permissions. | [required] |

### Return type

[**crate::models::MlExperimentPermissions**](MlExperimentPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentsget_history

> crate::models::MlGetMetricHistoryResponse experimentsget_history(metric_key, run_id, run_uuid, page_token, max_results)
Get history of a given metric within a run

Gets a list of all values for the specified metric for a given run.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metric_key** | **String** | Name of the metric. | [required] |
**run_id** | Option<**String**> | ID of the run from which to fetch metric values. Must be provided. |  |
**run_uuid** | Option<**String**> | [Deprecated, use run_id instead] ID of the run from which to fetch metric values. This field will be removed in a future MLflow version. |  |
**page_token** | Option<**String**> | Token indicating the page of metric histories to fetch. |  |
**max_results** | Option<**i32**> | Maximum number of Metric records to return per paginated request. Default is set to 25,000. If set higher than 25,000, a request Exception will be raised. |  |

### Return type

[**crate::models::MlGetMetricHistoryResponse**](MlGetMetricHistoryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentsget_run

> crate::models::MlGetRunResponse experimentsget_run(run_id, run_uuid)
Get a run

Gets the metadata, metrics, params, and tags for a run. In the case where multiple metrics with the same key are logged for a run, return only the value  with the latest timestamp.  If there are multiple values with the latest timestamp, return the maximum of these values. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** | ID of the run to fetch. Must be provided. | [required] |
**run_uuid** | Option<**String**> | [Deprecated, use run_id instead] ID of the run to fetch. This field will be removed in a future MLflow version. |  |

### Return type

[**crate::models::MlGetRunResponse**](MlGetRunResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentslist_artifacts

> crate::models::MlListArtifactsResponse experimentslist_artifacts(run_id, run_uuid, path, page_token)
Get all artifacts

List artifacts for a run.  Takes an optional `artifact_path` prefix. If it is specified, the response contains only artifacts with the specified prefix.\", 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | Option<**String**> | ID of the run whose artifacts to list. Must be provided. |  |
**run_uuid** | Option<**String**> | [Deprecated, use run_id instead] ID of the run whose artifacts to list. This field will be removed in a future MLflow version. |  |
**path** | Option<**String**> | Filter artifacts matching this path (a relative path from the root artifact directory). |  |
**page_token** | Option<**String**> | Token indicating the page of artifact results to fetch |  |

### Return type

[**crate::models::MlListArtifactsResponse**](MlListArtifactsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentslist_experiments

> crate::models::MlListExperimentsResponse experimentslist_experiments(view_type, max_results, page_token)
List experiments

Gets a list of all experiments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**view_type** | Option<**String**> | Qualifier for type of experiments to be returned. If unspecified, return only active experiments. |  |
**max_results** | Option<**i32**> | Maximum number of experiments desired. If `max_results` is unspecified, return all experiments. If `max_results` is too large, it'll be automatically capped at 1000. Callers of this endpoint are encouraged to pass max_results explicitly and leverage page_token to iterate through experiments. |  |
**page_token** | Option<**String**> | Token indicating the page of experiments to fetch |  |

### Return type

[**crate::models::MlListExperimentsResponse**](MlListExperimentsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentslog_batch

> serde_json::Value experimentslog_batch(ml_log_batch)
Log a batch

Logs a batch of metrics, params, and tags for a run. If any data failed to be persisted, the server will respond with an error (non-200 status code).  In case of error (due to internal server error or an invalid request), partial data may be written.  You can write metrics, params, and tags in interleaving fashion, but within a given entity type are guaranteed to follow the order specified in the request body.  The overwrite behavior for metrics,  params, and tags is as follows:  * Metrics: metric values are never overwritten.    Logging a metric (key, value, timestamp) appends to the set of values for the metric with the provided key.  * Tags: tag values can be overwritten by successive writes to the same tag key.    That is, if multiple tag values with the same key are provided in the same API request,    the last-provided tag value is written. Logging the same tag (key, value) is permitted. Specifically, logging a tag is idempotent.  * Parameters: once written, param values cannot be changed (attempting to overwrite a param value will result in an error).    However, logging the same param (key, value) is permitted. Specifically, logging a param is idempotent.    Request Limits   -------------------------------   A single JSON-serialized API request may be up to 1 MB in size and contain:   * No more than 1000 metrics,  params, and tags in total  * Up to 1000 metrics  * Up to 100  params  * Up to 100 tags   For example, a valid request might contain 900 metrics, 50 params, and 50 tags, but logging 900 metrics, 50 params,   and 51 tags is invalid.   The following limits also apply to metric, param, and tag keys and values:   * Metric keys, param keys, and tag keys can be up to 250 characters in length  * Parameter and tag values can be up to 250 characters in length  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_log_batch** | Option<[**MlLogBatch**](MlLogBatch.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentslog_inputs

> serde_json::Value experimentslog_inputs(ml_log_inputs)
Log inputs to a run

**NOTE:** Experimental: This API may change or be removed in a future release without warning.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_log_inputs** | Option<[**MlLogInputs**](MlLogInputs.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentslog_metric

> serde_json::Value experimentslog_metric(ml_log_metric)
Log a metric

Logs a metric for a run. A metric is a key-value pair (string key, float value) with an associated timestamp.  Examples include the various metrics that represent ML model accuracy. A metric can be logged multiple times. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_log_metric** | Option<[**MlLogMetric**](MlLogMetric.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentslog_model

> serde_json::Value experimentslog_model(ml_log_model)
Log a model

**NOTE:** Experimental: This API may change or be removed in a future release without warning.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_log_model** | Option<[**MlLogModel**](MlLogModel.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentslog_param

> serde_json::Value experimentslog_param(ml_log_param)
Log a param

Logs a param used for a run. A param is a key-value pair (string key, string value).  Examples include hyperparameters used for ML model training and constant dates and values used in an ETL pipeline.  A param can be logged only once for a run. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_log_param** | Option<[**MlLogParam**](MlLogParam.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentsrestore_experiment

> serde_json::Value experimentsrestore_experiment(ml_restore_experiment)
Restores an experiment

Restore an experiment marked for deletion. This also restores associated metadata, runs, metrics, params, and tags. If experiment uses FileStore, underlying artifacts associated with experiment are also restored.  Throws `RESOURCE_DOES_NOT_EXIST` if experiment was never created or was permanently deleted. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_restore_experiment** | Option<[**MlRestoreExperiment**](MlRestoreExperiment.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentsrestore_run

> serde_json::Value experimentsrestore_run(ml_restore_run)
Restore a run

Restores a deleted run.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_restore_run** | Option<[**MlRestoreRun**](MlRestoreRun.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentsrestore_runs

> crate::models::MlRestoreRunsResponse experimentsrestore_runs(ml_restore_runs)
Restore runs by deletion time

Bulk restore runs in an experiment that were deleted no earlier than the specified timestamp. Restores at most max_runs per request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_restore_runs** | Option<[**MlRestoreRuns**](MlRestoreRuns.md)> |  |  |

### Return type

[**crate::models::MlRestoreRunsResponse**](MlRestoreRunsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentssearch_experiments

> crate::models::MlSearchExperimentsResponse experimentssearch_experiments(ml_search_experiments)
Search experiments

Searches for experiments that satisfy specified search criteria.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_search_experiments** | Option<[**MlSearchExperiments**](MlSearchExperiments.md)> |  |  |

### Return type

[**crate::models::MlSearchExperimentsResponse**](MlSearchExperimentsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentssearch_runs

> crate::models::MlSearchRunsResponse experimentssearch_runs(ml_search_runs)
Search for runs

Searches for runs that satisfy expressions.   Search expressions can use `mlflowMetric` and `mlflowParam` keys.\", 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_search_runs** | Option<[**MlSearchRuns**](MlSearchRuns.md)> |  |  |

### Return type

[**crate::models::MlSearchRunsResponse**](MlSearchRunsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentsset_experiment_permissions

> crate::models::MlExperimentPermissions experimentsset_experiment_permissions(experiment_id, ml_experiment_permissions_request)
Set experiment permissions

Sets permissions on an experiment. Experiments can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experiment_id** | [**serde_json::Value**](.md) | The experiment for which to get or manage permissions. | [required] |
**ml_experiment_permissions_request** | Option<[**MlExperimentPermissionsRequest**](MlExperimentPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::MlExperimentPermissions**](MlExperimentPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentsset_experiment_tag

> serde_json::Value experimentsset_experiment_tag(ml_set_experiment_tag)
Set a tag

Sets a tag on an experiment. Experiment tags are metadata that can be updated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_set_experiment_tag** | Option<[**MlSetExperimentTag**](MlSetExperimentTag.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentsset_tag

> serde_json::Value experimentsset_tag(ml_set_tag)
Set a tag

Sets a tag on a run. Tags are run metadata that can be updated during a run and after a run completes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_set_tag** | Option<[**MlSetTag**](MlSetTag.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentsupdate_experiment

> serde_json::Value experimentsupdate_experiment(ml_update_experiment)
Update an experiment

Updates experiment metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_update_experiment** | Option<[**MlUpdateExperiment**](MlUpdateExperiment.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentsupdate_experiment_permissions

> crate::models::MlExperimentPermissions experimentsupdate_experiment_permissions(experiment_id, ml_experiment_permissions_request)
Update experiment permissions

Updates the permissions on an experiment. Experiments can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**experiment_id** | [**serde_json::Value**](.md) | The experiment for which to get or manage permissions. | [required] |
**ml_experiment_permissions_request** | Option<[**MlExperimentPermissionsRequest**](MlExperimentPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::MlExperimentPermissions**](MlExperimentPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## experimentsupdate_run

> crate::models::MlUpdateRunResponse experimentsupdate_run(ml_update_run)
Update a run

Updates run metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ml_update_run** | Option<[**MlUpdateRun**](MlUpdateRun.md)> |  |  |

### Return type

[**crate::models::MlUpdateRunResponse**](MlUpdateRunResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

