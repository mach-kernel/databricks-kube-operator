use assert_json_diff::{assert_json_matches_no_panic, Config};
use kube::core::object::HasSpec;
use kube::runtime::controller::Action;
use kube::{api::ListParams, runtime::Controller};
use kube::{Api, Error};
use serde_json::json;

use crate::context::Context;
use crate::crds::databricks_job::DatabricksJob;
use crate::error::DatabricksKubeError;
use crate::traits::remote_resource::RemoteResource;
use futures::StreamExt;
use kube::CustomResourceExt;
use kube::ResourceExt;
use std::sync::Arc;
use std::time::Duration;

use databricks_rust_jobs::models::{job::Job, JobsGet200Response};

async fn reconcile(resource: Arc<DatabricksJob>, context: Arc<Context>) -> Result<Action, Error> {
    log::info!(
        "Reconciling {} {}",
        DatabricksJob::api_resource().kind,
        resource.name_unchecked()
    );
    let rest_config = context.make_jobs_rest_config().await;

    if rest_config.is_none() {
        return Ok(Action::requeue(Duration::from_secs(15)));
    }

    let rest_config = rest_config.unwrap();
    let latest_remote = resource.remote_get_self(rest_config).next().await;

    if latest_remote.is_none() {
        log::info!(
            "Resource {} {} exists in K8S but not API",
            DatabricksJob::api_resource().kind,
            resource.name_unchecked()
        );
        log::info!(
            "Creating {} {} in Databricks",
            DatabricksJob::api_resource().kind,
            resource.name_unchecked()
        );
    }

    let latest_remote: Result<Job, DatabricksKubeError> = latest_remote.unwrap();
    if let Ok(latest_job) = latest_remote {
        if latest_job != resource.spec().job {
            log::info!(
                "Resource {} {} drifted!\nDiff:\n{}",
                DatabricksJob::api_resource().kind,
                resource.name_unchecked(),
                assert_json_matches_no_panic(
                    &latest_job,
                    &resource.spec().job,
                    assert_json_diff::Config::new(assert_json_diff::CompareMode::Strict)
                )
                .unwrap_err()
            );
        }

    } else if let Err(dke) = latest_remote {
        log::info!("Failed reconciling {}", dke);
    }

    Ok(Action::await_change())
}

pub async fn spawn_controller(context: Context) -> Result<(), DatabricksKubeError> {
    let root_kind_api = Api::<DatabricksJob>::default_namespaced(context.client.clone());

    Controller::new(root_kind_api, ListParams::default())
        .shutdown_on_signal()
        .run(
            reconcile,
            DatabricksJob::default_error_policy,
            context.into(),
        )
        .count()
        .await;

    Ok(())
}
