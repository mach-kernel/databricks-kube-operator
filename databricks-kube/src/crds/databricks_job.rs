use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::pin::Pin;
use std::sync::Arc;
use std::time::SystemTime;

use crate::traits::remote_api_status::RemoteAPIStatus;
use crate::util::hash_json_value;
use crate::{
    context::Context, error::DatabricksKubeError, traits::remote_api_resource::RemoteAPIResource,
    traits::rest_config::RestConfig,
};

use databricks_rust_jobs::models::{
    JobSettings, JobsRunsList200Response, RunLifeCycleState, RunState,
};
use databricks_rust_jobs::{
    apis::default_api,
    models::{
        job::Job, job_settings, jobs_create_request, JobsCreate200Response, JobsCreateRequest,
        JobsDeleteRequest, JobsGet200Response, JobsList200Response, JobsRunNowRequest,
        JobsUpdateRequest,
    },
};

use async_stream::try_stream;
use futures::{Future, FutureExt, Stream, StreamExt};

use k8s_openapi::DeepMerge;
use kube::core::object::HasSpec;
use kube::{CustomResource, ResourceExt};
use serde::{Deserialize, Serialize};
use serde_json::json;

use schemars::JsonSchema;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, JsonSchema)]
pub struct DatabricksJobStatus {
    pub latest_run_state: Option<RunState>,
}

// TODO: We added `NO_RUNS` to `RunLifeCycleState` because
// it was the laziest way to surface this, but should probably revisit
impl Default for DatabricksJobStatus {
    fn default() -> Self {
        Self {
            latest_run_state: Some(RunState {
                life_cycle_state: Some(RunLifeCycleState::Pending),
                ..RunState::default()
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
    pub job: Job,
    pub run: Option<JobsRunNowRequest>,
}

/// API -> CRD
impl From<Job> for DatabricksJob {
    fn from(job: Job) -> Self {
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

            let config = Job::get_rest_config(context.clone()).await.unwrap();

            // API says jobs are sorted by created_at
            let JobsRunsList200Response { runs, .. } = default_api::jobs_runs_list(
                &config,
                Some(true),
                Some(false),
                job_id,
                Some(0),
                None,
                None,
                None,
                None,
                None,
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
impl From<DatabricksJob> for Job {
    fn from(value: DatabricksJob) -> Self {
        value.spec().job.clone()
    }
}

impl DatabricksJob {
    fn hash_run_request(request: &JobsRunNowRequest, settings: Option<Box<JobSettings>>) -> u64 {
        let mut hasher = DefaultHasher::new();

        let request_as_value = serde_json::to_value(&request).unwrap();
        hash_json_value(&mut hasher, &request_as_value);

        if let Some(settings) = settings {
            let settings_as_value = serde_json::to_value(&settings).unwrap();
            hash_json_value(&mut hasher, &settings_as_value);
        }

        hasher.finish()
    }
}

impl RemoteAPIResource<Job> for DatabricksJob {
    fn remote_list_all(
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<Job, DatabricksKubeError>> + Send>> {
        try_stream! {
            let mut offset: i32 = 0;
            let config = Job::get_rest_config(context.clone()).await.unwrap();

            while let JobsList200Response {
                jobs,
                has_more,
                ..
            } = default_api::jobs_list(&config, None, Some(offset), None, Some(true)).await? {
                if let Some(jobs) = jobs {
                    offset = jobs.len() as i32;

                    for job in jobs {
                        yield job;
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
        context: Arc<Context>,
    ) -> Pin<Box<dyn Future<Output = Result<(), DatabricksKubeError>> + Send>> {
        let job_id = self.spec().job.job_id.clone();
        let job_settings = self.spec().job.settings.clone();
        let run_request = self.spec().run.clone();
        let self_name = self.name_unchecked();

        if run_request.is_none() {
            return async { Ok(()) }.boxed();
        }

        log::info!("Ensuring defined run for {}...", self.name_unchecked());

        async move {
            let config = Job::get_rest_config(context.clone()).await.unwrap();

            let mut run_request = JobsRunNowRequest {
                job_id,
                ..run_request.unwrap()
            };
            run_request.idempotency_token =
                Some(Self::hash_run_request(&run_request, job_settings).to_string());

            let triggered = default_api::jobs_run_now(&config, run_request).await?;

            log::info!(
                "Job {} reconciled run {}",
                &self_name,
                triggered.run_id.unwrap(),
            );

            Ok::<(), DatabricksKubeError>(())
        }
        .boxed()
    }

    fn remote_get(
        &self,
        context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<Job, DatabricksKubeError>> + Send>> {
        let job_id = self.spec().job.job_id;
        if job_id.is_none() { return tokio_stream::empty().boxed(); }

        try_stream! {
            let job_id = job_id.unwrap();
            let config = Job::get_rest_config(context.clone()).await.unwrap();

            let JobsGet200Response {
                job_id,
                creator_user_name,
                settings,
                created_time,
                ..
            } = default_api::jobs_get(&config, job_id).await?;

            yield Job {
                job_id,
                creator_user_name,
                settings,
                created_time
            }
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
            let config = Job::get_rest_config(context.clone()).await.unwrap();

            let JobsCreate200Response { job_id } = default_api::jobs_create(
                &config,

                /// TODO: unsupported atm
                // access_control_list: job.access_control_list
                JobsCreateRequest {
                    name: Some(self.name_unchecked()),
                    tags: job_settings.tags,
                    tasks: job_settings.tasks,
                    job_clusters: job_settings.job_clusters,
                    email_notifications: job_settings.email_notifications,
                    timeout_seconds: job_settings.timeout_seconds,
                    schedule: job_settings.schedule,
                    max_concurrent_runs: job_settings.max_concurrent_runs,
                    git_source: job_settings.git_source,
                    format: job_settings.format.map(job_settings_to_create_format),
                    ..JobsCreateRequest::default()
                }
            ).await?;

            // The response only contains an ID, but there are other fields
            // that are API populated which we need, so read again.
            let created: Self = Job { job_id, ..job }.into();
            let created_job = created.remote_get(context.clone()).next().await.unwrap()?;

            let mut with_response = self.clone();
            with_response.spec.job = Job { job_id, ..created_job };
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
        if job_id.is_none() { return tokio_stream::empty().boxed(); }

        try_stream! {
            let config = Job::get_rest_config(context.clone()).await.unwrap();

            default_api::jobs_update(
                &config,

                /// TODO: unsupported atm
                // access_control_list: job.access_control_list
                JobsUpdateRequest {
                    job_id: job_id.unwrap(),
                    new_settings: job_settings,
                    ..JobsUpdateRequest::default()
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
            let config = Job::get_rest_config(context.clone()).await.unwrap();
            default_api::jobs_delete(
                &config,
                JobsDeleteRequest { job_id: job_id.unwrap(), }
            ).await?;

            yield ()
        }
        .boxed()
    }
}

fn job_settings_to_create_format(value: job_settings::Format) -> jobs_create_request::Format {
    match value {
        job_settings::Format::MultiTask => jobs_create_request::Format::MultiTask,
        job_settings::Format::SingleTask => jobs_create_request::Format::SingleTask,
    }
}
