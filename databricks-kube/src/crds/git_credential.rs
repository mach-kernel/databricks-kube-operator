use async_stream::try_stream;


use futures::{FutureExt, Stream, StreamExt, TryFutureExt};
use k8s_openapi::serde::{Deserialize, Serialize};
use kube::{core::object::HasSpec, CustomResource};
use schemars::JsonSchema;

use crate::{error::DatabricksKubeError, traits::synced_api_resource::SyncedAPIResource};

use databricks_rust_git_credentials::{
    apis::{configuration::Configuration, default_api},
    models::{GetCredentialResponse as APICredential},
};
use std::{pin::Pin, time::SystemTime};

use databricks_rust_git_credentials::models::GetCredentialsResponse;
use crate::rest_config::RestConfig;
use crate::context::Context;


#[derive(Clone, CustomResource, Debug, Default, Deserialize, PartialEq, Serialize, JsonSchema)]
#[kube(
    group = "com.dstancu.databricks",
    version = "v1",
    kind = "GitCredential",
    derive = "Default",
    namespaced
)]
pub struct GitCredentialSpec {
    pub credential: APICredential,
    // The user provides an API token during a create request, but it
    // is otherwise no longer retrievable. Even with Helm/GitOps workflow,
    // the secret doesn't have to be checked in and could come from AWS 
    // SSM or secrets manager.
    pub secret_name: Option<String>,
}

/// API -> CRD
impl From<APICredential> for GitCredential {
    fn from(credential: APICredential) -> Self {
        let credential_name = if let Some(cid) = &credential.credential_id {
            cid.to_string()
        } else if let Some(git_username) = &credential.git_username {
            git_username.clone()
        } else {
            format!("noname-{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs())
        };

        Self::new(&credential_name, GitCredentialSpec { credential, secret_name: None })
    }
}

/// CRD -> API
impl From<GitCredential> for APICredential {
    fn from(value: GitCredential) -> Self {
        value.spec().credential.clone()
    }
}

impl SyncedAPIResource<APICredential, Configuration> for GitCredential {
    fn remote_list_all(
        context: Context,
    ) -> Pin<Box<dyn Stream<Item = Result<APICredential, DatabricksKubeError>> + Send>> {
        try_stream! {
            let config = APICredential::get_rest_config(context.clone()).await.unwrap();

            while let GetCredentialsResponse {
                credentials,
                ..
            } = default_api::get_git_credential_list(&config).await? {
                if let Some(credentials) = credentials {
                    for credential in credentials {
                        yield credential;
                    }
                }
            }
        }
        .boxed()
    }

    fn remote_get(
        &self,
        context: Context,
    ) -> Pin<Box<dyn Stream<Item = Result<APICredential, DatabricksKubeError>> + Send>> {
        let credential_id = self.spec().credential.credential_id;

        try_stream! {
            let config = APICredential::get_rest_config(context.clone()).await.unwrap();
            
            let res = default_api::get_git_credential(&config, &credential_id.unwrap().to_string()).map_err(
                |e| DatabricksKubeError::APIError(e.to_string())
            ).await?;

            yield res
        }
        .boxed()
    }

    // you have a problem -- go change the interface so this consumes a Context
    // you will also need to make a k8s api call to read the secrets
    fn remote_create(
        &self,
        _context: Context,
    ) -> Pin<Box<dyn Stream<Item = Result<Self, DatabricksKubeError>> + Send + '_>>
    where
        Self: Sized,
    {
        todo!();
        // let job = self.spec().job.clone();
        // let job_settings = job.settings.as_ref().unwrap().clone();

        // try_stream! {
        //     let JobsCreate200Response { job_id } = default_api::jobs_create(
        //         &config,

        //         /// TODO: unsupported atm
        //         // access_control_list: job.access_control_list
        //         Some(JobsCreateRequest {
        //             name: job_settings.name,
        //             tags: job_settings.tags,
        //             tasks: job_settings.tasks,
        //             job_clusters: job_settings.job_clusters,
        //             email_notifications: job_settings.email_notifications,
        //             timeout_seconds: job_settings.timeout_seconds,
        //             schedule: job_settings.schedule,
        //             max_concurrent_runs: job_settings.max_concurrent_runs,
        //             git_source: job_settings.git_source,
        //             format: job_settings.format.map(job_settings_to_create_format),
        //             ..JobsCreateRequest::default()
        //         })
        //     ).map_err(
        //         |e| DatabricksKubeError::APIError(e.to_string())
        //     ).await?;

        //     let mut with_response = self.clone();
        //     with_response.spec.job = Job { job_id, ..job };
        //     yield with_response
        // }
        // .boxed()
    }
}
