# \JobsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**jobscancel_all_runs**](JobsApi.md#jobscancel_all_runs) | **POST** /api/2.1/jobs/runs/cancel-all | Cancel all runs of a job
[**jobscancel_run**](JobsApi.md#jobscancel_run) | **POST** /api/2.1/jobs/runs/cancel | Cancel a job run
[**jobscreate**](JobsApi.md#jobscreate) | **POST** /api/2.1/jobs/create | Create a new job
[**jobsdelete**](JobsApi.md#jobsdelete) | **POST** /api/2.1/jobs/delete | Delete a job
[**jobsdelete_run**](JobsApi.md#jobsdelete_run) | **POST** /api/2.1/jobs/runs/delete | Delete a job run
[**jobsexport_run**](JobsApi.md#jobsexport_run) | **GET** /api/2.1/jobs/runs/export | Export and retrieve a job run
[**jobsget**](JobsApi.md#jobsget) | **GET** /api/2.1/jobs/get | Get a single job
[**jobsget_job_permission_levels**](JobsApi.md#jobsget_job_permission_levels) | **GET** /api/2.0/permissions/jobs/{job_id}/permissionLevels | Get job permission levels
[**jobsget_job_permissions**](JobsApi.md#jobsget_job_permissions) | **GET** /api/2.0/permissions/jobs/{job_id} | Get job permissions
[**jobsget_run**](JobsApi.md#jobsget_run) | **GET** /api/2.1/jobs/runs/get | Get a single job run
[**jobsget_run_output**](JobsApi.md#jobsget_run_output) | **GET** /api/2.1/jobs/runs/get-output | Get the output for a single run
[**jobslist**](JobsApi.md#jobslist) | **GET** /api/2.1/jobs/list | List jobs
[**jobslist_runs**](JobsApi.md#jobslist_runs) | **GET** /api/2.1/jobs/runs/list | List job runs
[**jobsrepair_run**](JobsApi.md#jobsrepair_run) | **POST** /api/2.1/jobs/runs/repair | Repair a job run
[**jobsreset**](JobsApi.md#jobsreset) | **POST** /api/2.1/jobs/reset | Overwrites all settings for a job
[**jobsrun_now**](JobsApi.md#jobsrun_now) | **POST** /api/2.1/jobs/run-now | Trigger a new job run
[**jobsset_job_permissions**](JobsApi.md#jobsset_job_permissions) | **PUT** /api/2.0/permissions/jobs/{job_id} | Set job permissions
[**jobssubmit**](JobsApi.md#jobssubmit) | **POST** /api/2.1/jobs/runs/submit | Create and trigger a one-time run
[**jobsupdate**](JobsApi.md#jobsupdate) | **POST** /api/2.1/jobs/update | Partially update a job
[**jobsupdate_job_permissions**](JobsApi.md#jobsupdate_job_permissions) | **PATCH** /api/2.0/permissions/jobs/{job_id} | Update job permissions



## jobscancel_all_runs

> serde_json::Value jobscancel_all_runs(jobs_cancel_all_runs)
Cancel all runs of a job

Cancels all active runs of a job. The runs are canceled asynchronously, so it doesn't prevent new runs from being started. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_cancel_all_runs** | [**JobsCancelAllRuns**](JobsCancelAllRuns.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobscancel_run

> serde_json::Value jobscancel_run(jobs_cancel_run)
Cancel a job run

Cancels a job run. The run is canceled asynchronously, so it may still be running when this request completes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_cancel_run** | [**JobsCancelRun**](JobsCancelRun.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobscreate

> crate::models::Jobscreate200Response jobscreate(jobs_create_job)
Create a new job

Create a new job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_create_job** | [**JobsCreateJob**](JobsCreateJob.md) |  | [required] |

### Return type

[**crate::models::Jobscreate200Response**](Jobscreate_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobsdelete

> serde_json::Value jobsdelete(jobs_delete_job)
Delete a job

Deletes a job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_delete_job** | [**JobsDeleteJob**](JobsDeleteJob.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobsdelete_run

> serde_json::Value jobsdelete_run(jobs_delete_run)
Delete a job run

Deletes a non-active run. Returns an error if the run is active.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_delete_run** | [**JobsDeleteRun**](JobsDeleteRun.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobsexport_run

> crate::models::JobsExportRunOutput jobsexport_run(run_id, views_to_export)
Export and retrieve a job run

Export and retrieve the job run task.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **i64** | The canonical identifier for the run. This field is required. | [required] |
**views_to_export** | Option<[**JobsViewsToExport**](.md)> | Which views to export (CODE, DASHBOARDS, or ALL). Defaults to CODE. |  |

### Return type

[**crate::models::JobsExportRunOutput**](JobsExportRunOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobsget

> crate::models::JobsJob jobsget(job_id)
Get a single job

Retrieves the details for a single job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **i64** | The canonical identifier of the job to retrieve information about. This field is required. | [required] |

### Return type

[**crate::models::JobsJob**](JobsJob.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobsget_job_permission_levels

> crate::models::JobsGetJobPermissionLevelsResponse jobsget_job_permission_levels(job_id)
Get job permission levels

Gets the permission levels that a user can have on an object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | The job for which to get or manage permissions. | [required] |

### Return type

[**crate::models::JobsGetJobPermissionLevelsResponse**](JobsGetJobPermissionLevelsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobsget_job_permissions

> crate::models::JobsJobPermissions jobsget_job_permissions(job_id)
Get job permissions

Gets the permissions of a job. Jobs can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | [**serde_json::Value**](.md) | The job for which to get or manage permissions. | [required] |

### Return type

[**crate::models::JobsJobPermissions**](JobsJobPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobsget_run

> crate::models::JobsRun jobsget_run(run_id, include_history)
Get a single job run

Retrieve the metadata of a run.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **i64** | The canonical identifier of the run for which to retrieve the metadata. This field is required.  | [required] |
**include_history** | Option<**bool**> | Whether to include the repair history in the response. |  |

### Return type

[**crate::models::JobsRun**](JobsRun.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobsget_run_output

> crate::models::JobsRunOutput jobsget_run_output(run_id)
Get the output for a single run

Retrieve the output and metadata of a single task run. When a notebook task returns a value through the `Dbutilsnotebook.exit()` call, you can use this endpoint to retrieve that value. Databricks restricts this API to returning the first 5 MB of the output. To return a larger result, you can store job results in a cloud storage service.  This endpoint validates that the __run_id__ parameter is valid and returns an HTTP status code 400 if the __run_id__ parameter is invalid. Runs are automatically removed after 60 days. If you to want to reference them beyond 60 days, you must save old run results before they expire. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **i64** | The canonical identifier for the run. This field is required. | [required] |

### Return type

[**crate::models::JobsRunOutput**](JobsRunOutput.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobslist

> crate::models::JobsListJobsResponse jobslist(limit, page_token, offset, name, expand_tasks)
List jobs

Retrieves a list of jobs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of jobs to return. This value must be greater than 0 and less or equal to 100. The default value is 20. |  |[default to 20]
**page_token** | Option<**String**> | Use `next_page_token` or `prev_page_token` returned from the previous request to list the next or previous page of jobs respectively. |  |
**offset** | Option<**i32**> | The offset of the first job to return, relative to the most recently created job.  Deprecated since June 2023. Use `page_token` to iterate through the pages instead.  |  |[default to 0]
**name** | Option<**String**> | A filter on the list based on the exact (case insensitive) job name. |  |
**expand_tasks** | Option<**bool**> | Whether to include task and cluster details in the response. |  |[default to false]

### Return type

[**crate::models::JobsListJobsResponse**](JobsListJobsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobslist_runs

> crate::models::JobsListRunsResponse jobslist_runs(active_only, completed_only, job_id, page_token, offset, limit, run_type, expand_tasks, start_time_from, start_time_to)
List job runs

List runs in descending order by start time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**active_only** | Option<**bool**> | If active_only is `true`, only active runs are included in the results; otherwise, lists both active and completed runs. An active run is a run in the `PENDING`, `RUNNING`, or `TERMINATING`. This field cannot be `true` when completed_only is `true`.  |  |[default to false]
**completed_only** | Option<**bool**> | If completed_only is `true`, only completed runs are included in the results; otherwise, lists both active and completed runs. This field cannot be `true` when active_only is `true`.  |  |[default to false]
**job_id** | Option<**i64**> | The job for which to list runs. If omitted, the Jobs service lists runs from all jobs. |  |
**page_token** | Option<**String**> | Use `next_page_token` or `prev_page_token` returned from the previous request to list the next or previous page of runs respectively. |  |
**offset** | Option<**i32**> | The offset of the first run to return, relative to the most recent run.  Deprecated since June 2023. Use `page_token` to iterate through the pages instead.  |  |[default to 0]
**limit** | Option<**i32**> | The number of runs to return. This value must be greater than 0 and less than 25. The default value is 25. If a request specifies a limit of 0, the service instead uses the maximum limit.  |  |[default to 25]
**run_type** | Option<**String**> | The type of runs to return. For a description of run types, see :method:jobs/getRun. |  |
**expand_tasks** | Option<**bool**> | Whether to include task and cluster details in the response. |  |[default to false]
**start_time_from** | Option<**i32**> | Show runs that started _at or after_ this value. The value must be a UTC timestamp in milliseconds. Can be combined with _start_time_to_ to filter by a time range.  |  |
**start_time_to** | Option<**i32**> | Show runs that started _at or before_ this value. The value must be a UTC timestamp in milliseconds. Can be combined with _start_time_from_ to filter by a time range.  |  |

### Return type

[**crate::models::JobsListRunsResponse**](JobsListRunsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobsrepair_run

> crate::models::JobsrepairRun200Response jobsrepair_run(jobs_repair_run)
Repair a job run

Re-run one or more tasks. Tasks are re-run as part of the original job run. They use the current job and task settings, and can be viewed in the history for the original job run. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_repair_run** | [**JobsRepairRun**](JobsRepairRun.md) |  | [required] |

### Return type

[**crate::models::JobsrepairRun200Response**](JobsrepairRun_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobsreset

> serde_json::Value jobsreset(jobs_reset_job)
Overwrites all settings for a job

Overwrites all the settings for a specific job. Use the Update endpoint to update job settings partially.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_reset_job** | [**JobsResetJob**](JobsResetJob.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobsrun_now

> crate::models::JobsRunNowResponse jobsrun_now(jobs_run_now)
Trigger a new job run

Run a job and return the `run_id` of the triggered run.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_run_now** | [**JobsRunNow**](JobsRunNow.md) |  | [required] |

### Return type

[**crate::models::JobsRunNowResponse**](JobsRunNowResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobsset_job_permissions

> crate::models::JobsJobPermissions jobsset_job_permissions(job_id, jobs_job_permissions_request)
Set job permissions

Sets permissions on a job. Jobs can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | [**serde_json::Value**](.md) | The job for which to get or manage permissions. | [required] |
**jobs_job_permissions_request** | Option<[**JobsJobPermissionsRequest**](JobsJobPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::JobsJobPermissions**](JobsJobPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobssubmit

> crate::models::JobsSubmitRunResponse jobssubmit(jobs_submit_run)
Create and trigger a one-time run

Submit a one-time run. This endpoint allows you to submit a workload directly without creating a job. Runs submitted using this endpoint don’t display in the UI. Use the `jobs/runs/get` API to check the run state after the job is submitted. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_submit_run** | [**JobsSubmitRun**](JobsSubmitRun.md) |  | [required] |

### Return type

[**crate::models::JobsSubmitRunResponse**](JobsSubmitRunResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobsupdate

> serde_json::Value jobsupdate(jobs_update_job)
Partially update a job

Add, update, or remove specific settings of an existing job. Use the ResetJob to overwrite all job settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jobs_update_job** | [**JobsUpdateJob**](JobsUpdateJob.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jobsupdate_job_permissions

> crate::models::JobsJobPermissions jobsupdate_job_permissions(job_id, jobs_job_permissions_request)
Update job permissions

Updates the permissions on a job. Jobs can inherit permissions from their root object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | [**serde_json::Value**](.md) | The job for which to get or manage permissions. | [required] |
**jobs_job_permissions_request** | Option<[**JobsJobPermissionsRequest**](JobsJobPermissionsRequest.md)> |  |  |

### Return type

[**crate::models::JobsJobPermissions**](JobsJobPermissions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

