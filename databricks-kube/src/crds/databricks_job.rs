use async_stream::try_stream;

use databricks_rust_jobs::models::{job::Job, JobsGet200Response};
use futures::{FutureExt, Stream, StreamExt, TryFutureExt};
use k8s_openapi::serde::{Deserialize, Serialize};
use kube::{core::object::HasSpec, CustomResource};
use schemars::JsonSchema;

use crate::{error::DatabricksKubeError, traits::remote_resource::RemoteResource};

use databricks_rust_jobs::{
    apis::{configuration::Configuration, default_api},
    models::JobsList200Response,
};
use std::pin::Pin;


#[derive(Clone, CustomResource, Debug, Default, Deserialize, PartialEq, Serialize, JsonSchema)]
#[kube(
    group = "com.dstancu",
    version = "v1",
    kind = "DatabricksJob",
    derive = "Default",
    namespaced
)]
pub struct DatabricksJobSpec {
    pub job: Job,
}

/// API -> CRD
impl From<Job> for DatabricksJob {
    fn from(job: Job) -> Self {
        let k8s_name = job
            .settings
            .iter()
            .flat_map(|s| s.name.clone())
            .next()
            .unwrap();

        Self::new(&k8s_name, DatabricksJobSpec { job })
    }
}

impl RemoteResource<Job> for DatabricksJob {
    fn remote_list_all(
        config: Configuration,
    ) -> Pin<Box<dyn Stream<Item = Result<Job, DatabricksKubeError>> + Send>> {
        try_stream! {
            let mut offset: i32 = 0;

            while let JobsList200Response {
                jobs,
                has_more,
                ..
            } = default_api::jobs_list(&config, None, Some(offset), Some(true)).await? {
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

    fn remote_get_self(
        &self,
        config: Configuration,
    ) -> Pin<Box<dyn Stream<Item = Result<Job, DatabricksKubeError>> + Send>> {
        let job_id = self.spec().job.job_id;

        try_stream! {
            let JobsGet200Response {
                job_id,
                creator_user_name,
                settings,
                created_time,
                ..
            } = default_api::jobs_get(&config, job_id).map_err(
                |e| DatabricksKubeError::APIError(e.to_string())
            ).await?;

            yield Job {
                job_id,
                creator_user_name,
                settings,
                created_time
            }
        }
        .boxed()
    }
}
