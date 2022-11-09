mod common;
use common::fake_resource::{FakeAPIResource, FakeResource, FakeResourceSpec};
use k8s_openapi::{
    api::core::v1::{ConfigMap, Secret},
};


use std::sync::Arc;
use std::{collections::BTreeMap, pin::Pin};

use databricks_kube::{
    context::Context, error::DatabricksKubeError, traits::synced_api_resource::SyncedAPIResource,
};

use async_stream::try_stream;
use futures::{Stream, StreamExt};
use kube::{
    core::object::HasSpec,
    runtime::reflector::{self, store::Writer, Store},
    Client, Resource,
};

use flurry::HashMap;
use lazy_static::lazy_static;


use hyper::body::HttpBody;
use hyper::Body;
use k8s_openapi::http::{Request, Response};
use tower_test::mock;

use common::mock_k8s::{
    mock_fake_resource_created, mock_fake_resource_updated, serve_fake_resource,
};

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

/// When the resource is created in Kubernetes
#[tokio::test]
async fn test_controller_lifecycle_create() {
    let (mock_service, mut handle) = mock::pair::<Request<Body>, Response<Body>>();

    let created_api_resource = FakeAPIResource {
        id: 1,
        description: None,
    };

    let mut created_resource: FakeResource = FakeResource::new(
        "test",
        FakeResourceSpec {
            api_resource: created_api_resource.clone(),
        },
    );

    created_resource.meta_mut().resource_version = Some("1".to_string());
    created_resource.meta_mut().annotations = Some({
        let mut annots = BTreeMap::new();
        annots.insert(
            "databricks-operator/owner".to_string(),
            "operator".to_string(),
        );
        annots
    });

    let kube_server = tokio::spawn(async move {
        loop {
            mock_fake_resource_created(&mut handle, created_resource.clone()).await
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
        created_api_resource
    );

    kube_server.abort();
    TEST_STORE.pin().clear();
}

/// When an owned resource is updated in Kubernetes
#[tokio::test]
async fn test_controller_lifecycle_update_kube_owned() {
    let (mock_service, mut handle) = mock::pair::<Request<Body>, Response<Body>>();

    let mut resource: FakeResource = FakeResource::new(
        "test",
        FakeResourceSpec {
            api_resource: FakeAPIResource {
                id: 1,
                description: None,
            },
        },
    );

    resource.meta_mut().resource_version = Some("1".to_string());
    resource.meta_mut().annotations = Some({
        let mut annots = BTreeMap::new();
        annots.insert(
            "databricks-operator/owner".to_string(),
            "operator".to_string(),
        );
        annots
    });

    let updated_api_resource = FakeAPIResource {
        id: 1,
        description: Some("foobar".to_string()),
    };

    let mut updated_resource = FakeResource::new(
        "test",
        FakeResourceSpec {
            api_resource: updated_api_resource.clone(),
        },
    );

    updated_resource.meta_mut().resource_version = Some("2".to_string());

    let kube_server = tokio::spawn(async move {
        loop {
            mock_fake_resource_updated(&mut handle, resource.clone(), updated_resource.clone())
                .await;
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
        updated_api_resource,
    );

    kube_server.abort();
    TEST_STORE.pin().clear();
}

/// When the API resource is updated for an owned resource
#[tokio::test]
async fn test_controller_lifecycle_update_api_owned() {
    // Begin with a CRD owned by the operator
    let mut resource: FakeResource = FakeResource::new(
        "foo",
        FakeResourceSpec {
            api_resource: FakeAPIResource {
                id: 42,
                description: None,
            },
        },
    );

    resource.meta_mut().annotations = Some({
        let mut annots = BTreeMap::new();
        annots.insert(
            "databricks-operator/owner".to_string(),
            "operator".to_string(),
        );
        annots
    });

    // Remote has a different value for "description"
    let updated_resource = FakeAPIResource {
        description: Some("hello".to_string()),
        ..resource.spec().api_resource
    };
    TEST_STORE.pin().insert(42, updated_resource.clone());

    let (mock_service, mut handle) = mock::pair::<Request<Body>, Response<Body>>();

    // Kube has a different version
    let kube_server = tokio::spawn(async move {
        loop {
            serve_fake_resource(
                &mut handle,
                resource.clone(),
                // assertion made during PUT call
                resource.spec().api_resource.clone(),
            )
            .await;
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

    // It reconciled successfully
    let reconciled = controller.next().await;
    assert!(reconciled.unwrap().is_ok());

    kube_server.abort();
    TEST_STORE.pin().clear();
}