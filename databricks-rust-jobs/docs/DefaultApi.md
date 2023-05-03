# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**jobs_create**](DefaultApi.md#jobs_create) | **POST** /2.1/jobs/create | Create a new job
[**jobs_delete**](DefaultApi.md#jobs_delete) | **POST** /2.1/jobs/delete | Delete a job
[**jobs_get**](DefaultApi.md#jobs_get) | **GET** /2.1/jobs/get | Get a single job
[**jobs_list**](DefaultApi.md#jobs_list) | **GET** /2.1/jobs/list | List all jobs
[**jobs_reset**](DefaultApi.md#jobs_reset) | **POST** /2.1/jobs/reset | Overwrites all settings for a job
[**jobs_run_now**](DefaultApi.md#jobs_run_now) | **POST** /2.1/jobs/run-now | Trigger a new job run
[**jobs_runs_cancel**](DefaultApi.md#jobs_runs_cancel) | **POST** /2.1/jobs/runs/cancel | Cancel a job run
[**jobs_runs_cancel_all**](DefaultApi.md#jobs_runs_cancel_all) | **POST** /2.1/jobs/runs/cancel-all | Cancel all runs of a job
[**jobs_runs_delete**](DefaultApi.md#jobs_runs_delete) | **POST** /2.1/jobs/runs/delete | Delete a job run
[**jobs_runs_export**](DefaultApi.md#jobs_runs_export) | **GET** /2.0/jobs/runs/export | Export and retrieve a job run
[**jobs_runs_get**](DefaultApi.md#jobs_runs_get) | **GET** /2.1/jobs/runs/get | Get a single job run
[**jobs_runs_get_output**](DefaultApi.md#jobs_runs_get_output) | **GET** /2.1/jobs/runs/get-output | Get the output for a single run
[**jobs_runs_list**](DefaultApi.md#jobs_runs_list) | **GET** /2.1/jobs/runs/list | List runs for a job
[**jobs_runs_repair**](DefaultApi.md#jobs_runs_repair) | **POST** /2.1/jobs/runs/repair | Repair a job run
[**jobs_runs_submit**](DefaultApi.md#jobs_runs_submit) | **POST** /2.1/jobs/runs/submit | Create and trigger a one-time run
[**jobs_update**](DefaultApi.md#jobs_update) | **POST** /2.1/jobs/update | Partially updates a job



## jobs_create

> crate::models::JobsCreate200Response jobs_create(jobs_create_request)
Create a new job

Create a new job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_create_request** | [**JobsCreateRequest**](JobsCreateRequest.md) |  | [required] |

### Return type

[**crate::models::JobsCreate200Response**](JobsCreate_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_delete

> serde_json::Value jobs_delete(jobs_delete_request)
Delete a job

Deletes a job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_delete_request** | [**JobsDeleteRequest**](JobsDeleteRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_get

> crate::models::JobsGet200Response jobs_get(job_id)
Get a single job

Retrieves the details for a single job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **i64** | The canonical identifier of the job to retrieve information about. This field is required. | [required] |

### Return type

[**crate::models::JobsGet200Response**](JobsGet_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_list

> crate::models::JobsList200Response jobs_list(limit, offset, name, expand_tasks)
List all jobs

Retrieves a list of jobs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of jobs to return. This value must be greater than 0 and less or equal to 25. The default value is 20. |  |[default to 20]
**offset** | Option<**i32**> | The offset of the first job to return, relative to the most recently created job. |  |[default to 0]
**name** | Option<**String**> | A filter on the list based on the exact (case insensitive) job name. |  |
**expand_tasks** | Option<**bool**> | Whether to include task and cluster details in the response. |  |[default to false]

### Return type

[**crate::models::JobsList200Response**](JobsList_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_reset

> serde_json::Value jobs_reset(jobs_reset_request)
Overwrites all settings for a job

Overwrites all the settings for a specific job. Use the Update endpoint to update job settings partially.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_reset_request** | [**JobsResetRequest**](JobsResetRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_run_now

> crate::models::JobsRunNow200Response jobs_run_now(jobs_run_now_request)
Trigger a new job run

Run a job and return the `run_id` of the triggered run.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_run_now_request** | [**JobsRunNowRequest**](JobsRunNowRequest.md) |  | [required] |

### Return type

[**crate::models::JobsRunNow200Response**](JobsRunNow_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_runs_cancel

> serde_json::Value jobs_runs_cancel(jobs_runs_cancel_request)
Cancel a job run

Cancels a job run. The run is canceled asynchronously, so it may still be running when this request completes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_runs_cancel_request** | [**JobsRunsCancelRequest**](JobsRunsCancelRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_runs_cancel_all

> serde_json::Value jobs_runs_cancel_all(jobs_runs_cancel_all_request)
Cancel all runs of a job

Cancels all active runs of a job. The runs are canceled asynchronously, so it doesn't prevent new runs from being started.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_runs_cancel_all_request** | [**JobsRunsCancelAllRequest**](JobsRunsCancelAllRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_runs_delete

> serde_json::Value jobs_runs_delete(jobs_runs_delete_request)
Delete a job run

Deletes a non-active run. Returns an error if the run is active.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_runs_delete_request** | [**JobsRunsDeleteRequest**](JobsRunsDeleteRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_runs_export

> crate::models::JobsRunsExport200Response jobs_runs_export(run_id, views_to_export)
Export and retrieve a job run

Export and retrieve the job run task.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **i64** | The canonical identifier for the run. This field is required. | [required] |
**views_to_export** | Option<[**ViewsToExport**](.md)> | Which views to export (CODE, DASHBOARDS, or ALL). Defaults to CODE. |  |

### Return type

[**crate::models::JobsRunsExport200Response**](JobsRunsExport_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_runs_get

> crate::models::JobsRunsGet200Response jobs_runs_get(run_id, include_history)
Get a single job run

Retrieve the metadata of a run.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **i64** | The canonical identifier of the run for which to retrieve the metadata. This field is required. | [required] |
**include_history** | Option<**bool**> | Whether to include the repair history in the response. |  |

### Return type

[**crate::models::JobsRunsGet200Response**](JobsRunsGet_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_runs_get_output

> crate::models::JobsRunsGetOutput200Response jobs_runs_get_output(run_id)
Get the output for a single run

Retrieve the output and metadata of a single task run. When a notebook task returns a value through the dbutils.notebook.exit() call, you can use this endpoint to retrieve that value. Databricks restricts this API to return the first 5 MB of the output. To return a larger result, you can store job results in a cloud storage service. This endpoint validates that the run_id parameter is valid and returns an HTTP status code 400 if the run_id parameter is invalid. Runs are automatically removed after 60 days. If you to want to reference them beyond 60 days, you must save old run results before they expire. To export using the UI, see Export job run results. To export using the Jobs API, see Runs export.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **i64** | The canonical identifier for the run. This field is required. | [required] |

### Return type

[**crate::models::JobsRunsGetOutput200Response**](JobsRunsGetOutput_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_runs_list

> crate::models::JobsRunsList200Response jobs_runs_list(active_only, completed_only, job_id, offset, limit, run_type, expand_tasks, start_time_from, start_time_to)
List runs for a job

List runs in descending order by start time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**active_only** | Option<**bool**> | If active_only is `true`, only active runs are included in the results; otherwise, lists both active and completed runs. An active run is a run in the `PENDING`, `RUNNING`, or `TERMINATING`. This field cannot be `true` when completed_only is `true`. |  |[default to false]
**completed_only** | Option<**bool**> | If completed_only is `true`, only completed runs are included in the results; otherwise, lists both active and completed runs. This field cannot be `true` when active_only is `true`. |  |[default to false]
**job_id** | Option<**i64**> | The job for which to list runs. If omitted, the Jobs service lists runs from all jobs. |  |
**offset** | Option<**i32**> | The offset of the first run to return, relative to the most recent run. |  |[default to 0]
**limit** | Option<**i32**> | The number of runs to return. This value must be greater than 0 and less than 25\\. The default value is 25\\. If a request specifies a limit of 0, the service instead uses the maximum limit. |  |[default to 25]
**run_type** | Option<**String**> | The type of runs to return. For a description of run types, see [Run](https://docs.databricks.com/dev-tools/api/latest/jobs.html#operation/JobsRunsGet). |  |
**expand_tasks** | Option<**bool**> | Whether to include task and cluster details in the response. |  |[default to false]
**start_time_from** | Option<**i32**> | Show runs that started _at or after_ this value. The value must be a UTC timestamp in milliseconds. Can be combined with _start_time_to_ to filter by a time range. |  |
**start_time_to** | Option<**i32**> | Show runs that started _at or before_ this value. The value must be a UTC timestamp in milliseconds. Can be combined with _start_time_from_ to filter by a time range. |  |

### Return type

[**crate::models::JobsRunsList200Response**](JobsRunsList_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_runs_repair

> crate::models::JobsRunsRepair200Response jobs_runs_repair(jobs_runs_repair_request)
Repair a job run

Re-run one or more tasks. Tasks are re-run as part of the original job run, use the current job and task settings, and can be viewed in the history for the original job run.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_runs_repair_request** | [**JobsRunsRepairRequest**](JobsRunsRepairRequest.md) |  | [required] |

### Return type

[**crate::models::JobsRunsRepair200Response**](JobsRunsRepair_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_runs_submit

> crate::models::JobsRunsSubmit200Response jobs_runs_submit(jobs_runs_submit_request)
Create and trigger a one-time run

Submit a one-time run. This endpoint allows you to submit a workload directly without creating a job. Use the `jobs/runs/get` API to check the run state after the job is submitted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_runs_submit_request** | [**JobsRunsSubmitRequest**](JobsRunsSubmitRequest.md) |  | [required] |

### Return type

[**crate::models::JobsRunsSubmit200Response**](JobsRunsSubmit_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobs_update

> serde_json::Value jobs_update(jobs_update_request)
Partially updates a job

Add, update, or remove specific settings of an existing job. Use the Reset endpoint to overwrite all job settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_update_request** | [**JobsUpdateRequest**](JobsUpdateRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

