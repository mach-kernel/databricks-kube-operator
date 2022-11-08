mod common;
use common::fake_resource::{FakeAPIResource, FakeResource};
use k8s_openapi::{
    api::core::v1::{ConfigMap, Secret},
    List,
};
use serde_json::Value;

use std::pin::Pin;
use std::sync::Arc;

use databricks_kube::{
    context::Context, error::DatabricksKubeError, traits::synced_api_resource::SyncedAPIResource,
};

use async_stream::try_stream;
use futures::{Stream, StreamExt};
use kube::{
    core::object::HasSpec,
    runtime::reflector::{self, store::Writer, Store},
    Client,
};

use flurry::HashMap;
use lazy_static::lazy_static;
use tower_test::mock::Handle;

use hyper::body::HttpBody;
use hyper::Body;
use k8s_openapi::http::{Request, Response};
use tower_test::mock;

lazy_static! {
    static ref TEST_STORE: HashMap<i64, FakeAPIResource> = HashMap::new();
}

impl SyncedAPIResource<FakeAPIResource, ()> for FakeResource {
    fn remote_list_all(
        _context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<FakeAPIResource, DatabricksKubeError>> + Send>> {
        let resources = TEST_STORE.pin();
        let resources: Vec<FakeAPIResource> = resources.values().map(Clone::clone).collect();

        try_stream! {
            for resource in resources {
                yield resource.clone();
            }
        }
        .boxed()
    }

    fn remote_get(
        &self,
        _context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<FakeAPIResource, DatabricksKubeError>> + Send>> {
        let found = TEST_STORE
            .pin()
            .get(&self.spec().api_resource.id)
            .map(Clone::clone);

        try_stream! {
            yield found.ok_or_else(|| DatabricksKubeError::APIError("Not found".to_string()))?;
        }
        .boxed()
    }

    fn remote_create(
        &self,
        _context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<FakeResource, DatabricksKubeError>> + Send + '_>>
    where
        Self: Sized,
    {
        let api_resource = self.spec().api_resource.clone();
        try_stream! {
            TEST_STORE.pin().insert(api_resource.id, api_resource.clone());
            yield self.clone();
        }
        .boxed()
    }

    fn remote_update(
        &self,
        _context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<FakeResource, DatabricksKubeError>> + Send + '_>>
    where
        Self: Sized,
    {
        try_stream! {
            TEST_STORE.pin().insert(self.spec().api_resource.id, self.spec().api_resource.clone());
            yield self.clone()
        }
        .boxed()
    }

    fn remote_delete(
        &self,
        _context: Arc<Context>,
    ) -> Pin<Box<dyn Stream<Item = Result<(), DatabricksKubeError>> + Send + '_>> {
        try_stream! {
            TEST_STORE.pin().remove_entry(&self.spec().api_resource.id);
            yield ()
        }
        .boxed()
    }
}

async fn mock_fake_resource_created(handle: &mut Handle<Request<Body>, Response<Body>>) {
    let (request, send) = handle.next_request().await.expect("Service not called");

    let body = match (
        request.method().as_str(),
        request.uri().path().to_string().as_str(),
    ) {
        ("GET", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources") => {
            let resource = match request.uri().query() {
                Some(q) if q.contains("watch") => {
                    serde_json::json!(
                        {
                            "type": "ADDED",
                            "object": {
                                "apiVersion": "com.dstancu.test/v1",
                                "kind": "FakeResource",
                                "metadata": {
                                    "name": "test",
                                    "resourceVersion": "2",
                                },
                                "spec": {
                                    "api_resource": {
                                        "id": 1
                                    }
                                }
                            }
                        }
                    )
                }
                _ => {
                    serde_json::json!(
                        {
                            "apiVersion": "v1",
                            "kind": "List",
                            "metadata": {
                                "resourceVersion": ""
                            },
                            "items": []
                        }
                    )
                }
            };

            serde_json::to_vec(&resource).unwrap()
        }
        ("PUT", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources/test") => {
            hyper::body::to_bytes(request.into_body())
                .await
                .unwrap()
                .into()
        }
        _ => panic!("Unexpected API request {:?}", request),
    };

    send.send_response(Response::builder().body(Body::from(body)).unwrap());
}

async fn mock_fake_resource_updated(handle: &mut Handle<Request<Body>, Response<Body>>) {
    let (request, send) = handle.next_request().await.expect("Service not called");

    let body = match (
        request.method().as_str(),
        request.uri().path().to_string().as_str(),
    ) {
        ("GET", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources") => {
            let resource = match request.uri().query() {
                Some(q) if q.contains("watch") => {
                    serde_json::json!(
                        {
                            "type": "MODIFIED",
                            "object": {
                                "apiVersion": "com.dstancu.test/v1",
                                "kind": "FakeResource",
                                "metadata": {
                                    "name": "test",
                                    "resourceVersion": "3",
                                },
                                "spec": {
                                    "api_resource": {
                                        "id": 1,
                                        "description": "foobar"
                                    }
                                }
                            }
                        }
                    )
                }
                _ => {
                    serde_json::json!(
                        {
                            "apiVersion": "v1",
                            "kind": "List",
                            "metadata": {
                                "resourceVersion": ""
                            },
                            "items": [
                                {
                                    "apiVersion": "com.dstancu.test/v1",
                                    "kind": "FakeResource",
                                    "metadata": {
                                        "name": "test",
                                        "resourceVersion": "2",
                                    },
                                    "spec": {
                                        "api_resource": {
                                            "id": 1
                                        }
                                    }
                                }
                            ]
                        }
                    )
                }
            };

            serde_json::to_vec(&resource).unwrap()
        }
        ("PUT", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources/test") => {
            hyper::body::to_bytes(request.into_body())
                .await
                .unwrap()
                .into()
        }
        _ => panic!("Unexpected API request {:?}", request),
    };

    send.send_response(Response::builder().body(Body::from(body)).unwrap());
}

#[tokio::test]
async fn test_controller_lifecycle_create() {
    TEST_STORE.pin().clear();

    let (mock_service, mut handle) = mock::pair::<Request<Body>, Response<Body>>();

    let kube_server = tokio::spawn(async move {
        loop {
            mock_fake_resource_created(&mut handle).await;
        }
    });

    let (configmap_store, _): (Store<ConfigMap>, Writer<ConfigMap>) = reflector::store();
    let (api_secret_store, _): (Store<Secret>, Writer<Secret>) = reflector::store();

    let kube_client = Client::new(mock_service, "default");

    let mut controller = FakeResource::controller(Context::new(
        kube_client,
        Arc::new(api_secret_store),
        Arc::new(configmap_store),
    ));

    // It reconciled successfully and the resources are in sync
    let reconciled = controller.next().await;
    assert!(reconciled.unwrap().is_ok());
    assert_eq!(
        TEST_STORE.pin().get(&1).unwrap().clone(),
        FakeAPIResource {
            id: 1,
            description: None
        }
    );

    kube_server.abort();
}

#[tokio::test]
async fn test_controller_lifecycle_update() {
    TEST_STORE.pin().clear();

    let (mock_service, mut handle) = mock::pair::<Request<Body>, Response<Body>>();

    let kube_server = tokio::spawn(async move {
        loop {
            mock_fake_resource_updated(&mut handle).await;
        }
    });

    let (configmap_store, _): (Store<ConfigMap>, Writer<ConfigMap>) = reflector::store();
    let (api_secret_store, _): (Store<Secret>, Writer<Secret>) = reflector::store();

    let kube_client = Client::new(mock_service, "default");

    let mut controller = FakeResource::controller(Context::new(
        kube_client,
        Arc::new(api_secret_store),
        Arc::new(configmap_store),
    ));

    // It reconciled successfully and the resources are in sync
    let reconciled = controller.next().await;
    assert!(reconciled.unwrap().is_ok());
    assert_eq!(
        TEST_STORE.pin().get(&1).unwrap().clone(),
        FakeAPIResource {
            id: 1,
            description: Some("foobar".to_string())
        }
    );

    kube_server.abort();
}
