mod common;

use std::{pin::Pin, sync::Arc};

use common::fake_resource::{
    FakeAPIResource, FakeAPIResourceStatus, FakeResource, FakeResourceSpec,
};
use common::mock_k8s::mock_list_fake_resource;

use databricks_kube::traits::remote_api_status::RemoteAPIStatus;
use databricks_kube::{context::Context, error::DatabricksKubeError};

use futures::{FutureExt, StreamExt};
use http::{Request, Response};
use k8s_openapi::api::core::v1::{ConfigMap, Secret};
use k8s_openapi::DeepMerge;
use kube::core::object::HasStatus;
use kube::{
    client::Body,
    runtime::reflector::{self, store::Writer, Store},
    Client,
};

use tower_test::mock;

impl DeepMerge for FakeAPIResourceStatus {
    fn merge_from(&mut self, other: Self) {
        self.foos = other.foos
    }
}

impl RemoteAPIStatus<FakeAPIResourceStatus> for FakeResource {
    fn remote_status(
        &self,
        _ctx: Arc<Context>,
    ) -> Pin<
        Box<
            dyn futures::Future<Output = Result<Option<FakeAPIResourceStatus>, DatabricksKubeError>>
                + Send,
        >,
    > {
        async { Ok(Some(FakeAPIResourceStatus { foos: Some(42) })) }.boxed()
    }
}

#[tokio::test]
async fn test_status_controller() {
    let mut resource: FakeResource = FakeResource::new(
        "foo",
        FakeResourceSpec {
            api_resource: FakeAPIResource {
                id: 1,
                description: None,
            },
        },
    );

    let initial_foos = 5;
    resource.status_mut().replace(FakeAPIResourceStatus {
        foos: Some(initial_foos),
    });

    let (mock_service, mut handle) = mock::pair::<Request<Body>, Response<Body>>();

    let kube_server = tokio::spawn(async move {
        loop {
            mock_list_fake_resource(
                &mut handle,
                resource.clone(),
                None,
                // assertion in tower server
                Some(FakeAPIResourceStatus { foos: Some(42) }),
            )
            .await;
        }
    });

    let (configmap_store, _): (Store<ConfigMap>, Writer<ConfigMap>) = reflector::store();
    let (api_secret_store, _): (Store<Secret>, Writer<Secret>) = reflector::store();

    let kube_client = Client::new(mock_service, "default");

    let mut controller = FakeResource::status_controller(Context::new(
        kube_client,
        Arc::new(api_secret_store),
        Arc::new(configmap_store),
    ));

    // It reconciled successfully
    let reconciled = controller.next().await;
    assert!(reconciled.unwrap().is_ok());

    kube_server.abort();
}
