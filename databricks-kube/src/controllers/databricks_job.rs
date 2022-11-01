use kube::{Api, Error};
use kube::runtime::controller::Action;
use kube::{runtime::Controller, api::ListParams};

use crate::crds::databricks_job::DatabricksJob;
use crate::error::DatabricksKubeError;
use crate::traits::remote_resource::RemoteResource;
use std::sync::Arc;
use crate::config::Config;
use futures::{StreamExt};


async fn reconcile(
    resource: Arc<DatabricksJob>,
    _context: Arc<Config>
) -> Result<Action, Error> {
    log::info!("Called with {:?}", resource);
    Ok(Action::await_change())
}

pub async fn spawn_controller(config: Config) -> Result<(), DatabricksKubeError> {
    let root_kind_api = Api::<DatabricksJob>::default_namespaced(config.client.clone());

    Controller::new(root_kind_api, ListParams::default())
        .shutdown_on_signal()
        .run(reconcile, DatabricksJob::default_error_policy, config.into())
        .count()
        .await;
    
    Ok(())
}