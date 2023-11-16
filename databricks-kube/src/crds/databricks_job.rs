use std::pin::Pin;
use std::sync::Arc;
use std::time::SystemTime;

use crate::traits::remote_api_status::RemoteAPIStatus;
use crate::{
    context::Context, error::DatabricksKubeError, traits::remote_api_resource::RemoteAPIResource,
    traits::rest_config::RestConfig,
};

use async_stream::try_stream;
use databricks_rust_openapi::models::{JobsRunState, JobsRunLifeCycleState, JobsListRunsResponse, JobsJob, JobsRunNow, JobsListJobsResponse, Jobscreate200Response, JobsCreateJob, JobsUpdateJob, JobsDeleteJob};
use databricks_rust_openapi::apis::jobs_api;
use futures::{Future, FutureExt, Stream, StreamExt};

use k8s_openapi::DeepMerge;
use kube::core::object::HasSpec;
use kube::{CustomResource, ResourceExt};
use serde::{Deserialize, Serialize};
use serde_json::json;

use schemars::JsonSchema;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct DatabricksJobStatus {
    pub latest_run_state: Option<JobsRunState>,
}

// TODO: We added `NO_RUNS` to `RunLifeCycleState` because
// it was the laziest way to surface this, but should probably revisit
impl Default for DatabricksJobStatus {
    fn default() -> Self {
        Self {
            latest_run_state: Some(JobsRunState {
                life_cycle_state: Some(JobsRunLifeCycleState::Pending),
                ..JobsRunState::default()
            }),
        }
    }
}

impl DeepMerge for DatabricksJobStatus {
    fn merge_from(&mut self, other: Self) {
        let mut self_state = json!(self.latest_run_state);
        let other_state = json!(other.latest_run_state);

        self_state.merge_from(other_state);
        self.latest_run_state = serde_json::from_value(self_state).unwrap();
    }
}

#[derive(Clone, CustomResource, Debug, Default, Deserialize, PartialEq, Serialize, JsonSchema)]
#[kube(
    group = "com.dstancu.databricks",
    version = "v1",
    kind = "DatabricksJob",
    derive = "Default",
    status = "DatabricksJobStatus",
    printcolumn = r#"{"name": "Status", "description": "Current life_cycle_state of the job", "jsonPath": ".status.latest_run_state.life_cycle_state", "type": "string"}"#,
    namespaced
)]
pub struct DatabricksJobSpec {
    pub job: JobsJob,
    pub run: Option<JobsRunNow>,
}

/// API -> CRD
impl From<JobsJob> for DatabricksJob {
    fn from(job: JobsJob) -> Self {
        let k8s_name = job
            .settings
            .iter()
            .flat_map(|s| s.name.clone())
            .next()
            .unwrap_or(format!(
                "noname-{}",
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            ));

        let run = None;

        Self::new(&k8s_name, DatabricksJobSpec { job, run })
    }
}

impl RemoteAPIStatus<DatabricksJobStatus> for DatabricksJob {
    fn remote_status(
        &self,
        context: Arc<Context>,
    ) -> Pin<
        Box<dyn Future<Output = Result<Option<DatabricksJobStatus>, DatabricksKubeError>> + Send>,
    > {
        let job_id = self.spec().job.job_id;

        async move {
            if job_id.is_none() {
                return Ok(None);
            }

            let config = JobsJob::get_rest_config(context.clone()).await.unwrap();

            // API says jobs are sorted by created_at
            let JobsListRunsResponse { runs, .. } = jobs_api::jobslist_runs(
                &config,
                Some(true),
                Some(false),
                job_id,
                None,
                None,
                None,
                None,
                None,
                None,
                None
            )
            .await?;

            let status = runs
                .iter()
                .flat_map(|runs| runs.first())
                .flat_map(|run| run.state.clone())
                .map(|state| *state.clone())
                .next()
                .map(|run_state| DatabricksJobStatus {
                    latest_run_state: Some(run_state),
                })
                .unwrap_or_default();

            Ok(Some(status))
        }
        .boxed()
    }
}

/// CRD -> API
impl From<DatabricksJob> for JobsJob {
    fn from(value: DatabricksJob) -> Self {
        value.spec().job.clone()
    }
}

impl RemoteAPIResource<JobsJob> for DatabricksJob {
    fn remote_list_all(
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<JobsJob, DatabricksKubeError>> + Send>> {
        try_stream! {
            let mut offset: i32 = 0;
            let config = JobsJob::get_rest_config(context.clone()).await.unwrap();

            while let JobsListJobsResponse {
                jobs,
                has_more,
                ..
            } = jobs_api::jobslist(&config, None, None, Some(offset), None, Some(true)).await? {
                if let Some(jobs) = jobs {
                    offset = jobs.len() as i32;

                    for job in jobs {
                        yield JobsJob { 
                            created_time: job.created_time,
                            creator_user_name: job.creator_user_name,
                            job_id: job.job_id,
                            settings: job.settings,
                            ..JobsJob::default()
                         };
                    }
                }

                let more = has_more.unwrap_or(false);
                if !more { break; }
            }
        }
        .boxed()
    }

    #[allow(irrefutable_let_patterns)]
    fn every_reconcile_owned(
        &self,
        _: Arc<Context>,
    ) -> Pin<Box<dyn Future<Output = Result<(), DatabricksKubeError>> + Send>> {
        async { Ok(()) }.boxed()
    }

    fn remote_get(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<JobsJob, DatabricksKubeError>> + Send>> {
        let job_id = self.spec().job.job_id;

        try_stream! {
            if job_id.is_none() {
                yield Err(DatabricksKubeError::APIError("Resource does not exist".to_string()))?;
            }

            let config = JobsJob::get_rest_config(context.clone()).await.unwrap();
            yield jobs_api::jobsget(&config, job_id.unwrap()).await?;
        }
        .boxed()
    }

    fn remote_create(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<Self, DatabricksKubeError>> + Send + '_>>
    where
        Self: Sized,
    {
        let job = self.spec().job.clone();
        let job_settings = job.settings.as_ref().unwrap().clone();

        try_stream! {
            let config = JobsJob::get_rest_config(context.clone()).await.unwrap();

            let Jobscreate200Response { job_id } = jobs_api::jobscreate(
                &config,

                /// TODO: unsupported atm
                // access_control_list: job.access_control_list
                JobsCreateJob {
                    name: Some(self.name_unchecked()),
                    tags: job_settings.tags,
                    tasks: job_settings.tasks,
                    job_clusters: job_settings.job_clusters,
                    email_notifications: job_settings.email_notifications,
                    timeout_seconds: job_settings.timeout_seconds,
                    schedule: job_settings.schedule,
                    max_concurrent_runs: job_settings.max_concurrent_runs,
                    git_source: job_settings.git_source,
                    format: job_settings.format,
                    continuous: job_settings.continuous,
                    ..JobsCreateJob::default()
                }
            ).await?;

            // The response only contains an ID, but there are other fields
            // that are API populated which we need, so read again.
            let created: Self = JobsJob { job_id, ..job }.into();
            let created_job = created.remote_get(context.clone()).next().await.unwrap()?;

            let mut with_response = self.clone();
            with_response.spec.job = JobsJob { job_id, ..created_job };
            yield with_response;
        }
        .boxed()
    }

    fn remote_update(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<Self, DatabricksKubeError>> + Send + '_>>
    where
        Self: Sized,
    {
        let job = self.spec().job.clone();
        let job_settings = job.settings.as_ref().cloned();
        let job_id = self.spec().job.job_id;

        try_stream! {
            if job_id.is_none() {
                yield Err(DatabricksKubeError::APIError("Resource does not exist".to_string()))?;
            }

            let config = JobsJob::get_rest_config(context.clone()).await.unwrap();

            jobs_api::jobsupdate(
                &config,

                /// TODO: unsupported atm
                // access_control_list: job.access_control_list
                JobsUpdateJob {
                    job_id: job_id.unwrap(),
                    new_settings: job_settings,
                    ..JobsUpdateJob::default()
                }
            ).await?;

            let mut with_response = self.clone();
            with_response.spec.job = self.remote_get(context.clone()).next().await.unwrap()?;
            yield with_response;
        }
        .boxed()
    }

    fn remote_delete(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<(), DatabricksKubeError>> + Send + '_>> {
        let job_id = self.spec().job.job_id;

        try_stream! {
            if job_id.is_none() {
                yield Err(DatabricksKubeError::APIError("Resource does not exist".to_string()))?;
            }

            let config = JobsJob::get_rest_config(context.clone()).await.unwrap();
            jobs_api::jobsdelete(
                &config,
                JobsDeleteJob { job_id: job_id.unwrap(), }
            ).await?;

            yield ()
        }
        .boxed()
    }
}

// fn job_settings_to_create_format(value: job_settings::Format) -> jobs_create_request::Format {
//     match value {
//         job_settings::Format::MultiTask => jobs_create_request::Format::MultiTask,
//         job_settings::Format::SingleTask => jobs_create_request::Format::SingleTask,
//     }
// }
