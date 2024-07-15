#![allow(dead_code)]


use crate::common::fake_resource::FakeResource;

use kube::{client::Body, Resource};
use tower_test::mock::Handle;

use http::{Request, Response};

use super::fake_resource::{FakeAPIResource, FakeAPIResourceStatus};

/**
 * Big thanks to this thread: https://github.com/kube-rs/kube/issues/429
 * @kazk's tower testing scaffold is extremely helpful!
 */

pub async fn mock_fake_resource_created(
    handle: &mut Handle<Request<Body>, Response<Body>>,
    created_resource: FakeResource,
) {
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
                            "object": created_resource,
                        }
                    )
                }
                _ => {
                    serde_json::json!(
                        {
                            "apiVersion": "v1",
                            "kind": "List",
                            "metadata": {
                                "resourceVersion": "1"
                            },
                            "items": []
                        }
                    )
                }
            };

            serde_json::to_vec(&resource).unwrap()
        }
        ("PUT", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources/test") => {
            request.into_body().collect_bytes().await.unwrap().into()
        }
        _ => panic!("Unexpected API request {:?}", request),
    };

    send.send_response(Response::builder().body(Body::from(body)).unwrap());
}

pub async fn mock_fake_resource_updated_kube(
    handle: &mut Handle<Request<Body>, Response<Body>>,
    resource: FakeResource,
    modified_resource: FakeResource,
    assert_put: FakeAPIResource,
    watch_update: Option<String>,
) {
    let (request, send) = handle.next_request().await.expect("Service not called");

    let body = match (
        request.method().as_str(),
        request.uri().path().to_string().as_str(),
    ) {
        ("GET", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources") => {
            let response = match request.uri().query() {
                Some(q) if q.contains("watch") => {
                    serde_json::json!(
                        {
                            "type": watch_update.unwrap_or("MODIFIED".to_string()),
                            "object": modified_resource
                        }
                    )
                }
                _ => {
                    serde_json::json!(
                        {
                            "apiVersion": "v1",
                            "kind": "List",
                            "metadata": {
                                "resourceVersion": "1"
                            },
                            "items": [
                                resource
                            ]
                        }
                    )
                }
            };

            serde_json::to_vec(&response).unwrap()
        }
        ("PUT", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources/test") => {
            let parsed: FakeResource =
                serde_json::from_slice(&request.into_body().collect_bytes().await.unwrap())
                    .unwrap();

            assert_eq!(assert_put, parsed.spec.api_resource,);

            serde_json::to_vec(&parsed).unwrap()
        }
        _ => panic!("Unexpected API request {:?}", request),
    };

    send.send_response(Response::builder().body(Body::from(body)).unwrap());
}

pub async fn mock_list_fake_resource(
    handle: &mut Handle<Request<Body>, Response<Body>>,
    resource: FakeResource,
    assert_put_resource: Option<FakeAPIResource>,
    assert_put_status: Option<FakeAPIResourceStatus>,
) {
    let (request, send) = handle.next_request().await.expect("Service not called");

    let body = match (
        request.method().as_str(),
        request.uri().path().to_string().as_str(),
    ) {
        ("GET", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources") => {
            match request.uri().query() {
                Some(q) if q.contains("watch") => vec![],
                _ => {
                    let list = serde_json::json!(
                        {
                            "apiVersion": "v1",
                            "kind": "List",
                            "metadata": {
                                "resourceVersion": "1"
                            },
                            "items": [
                                resource,
                            ]
                        }
                    );
                    serde_json::to_vec(&list).unwrap()
                }
            }
        }
        ("PUT", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources/foo") => {
            let parsed: FakeResource =
                serde_json::from_slice(&request.into_body().collect_bytes().await.unwrap())
                    .unwrap();

            assert_eq!(assert_put_resource.unwrap(), parsed.spec.api_resource);
            serde_json::to_vec(&parsed).unwrap()
        }
        ("PUT", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources/foo/status") => {
            let parsed: FakeResource =
                serde_json::from_slice(&request.into_body().collect_bytes().await.unwrap())
                    .unwrap();

            assert_eq!(assert_put_status.unwrap(), parsed.status.clone().unwrap());
            serde_json::to_vec(&parsed).unwrap()
        }
        _ => panic!("Unexpected API request {:?}", request),
    };

    send.send_response(Response::builder().body(Body::from(body)).unwrap());
}

pub async fn mock_fake_resource_deleted(
    handle: &mut Handle<Request<Body>, Response<Body>>,
    mut resource: FakeResource,
) {
    let (request, send) = handle.next_request().await.expect("Service not called");

    let body = match (
        request.method().as_str(),
        request.uri().path().to_string().as_str(),
    ) {
        ("GET", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources") => {
            match request.uri().query() {
                Some(q) if q.contains("watch") => serde_json::to_vec(&serde_json::json!(
                    {
                        "type": "MODIFIED",
                        "object": resource,
                    }
                ))
                .unwrap(),
                _ => {
                    let list = serde_json::json!(
                        {
                            "apiVersion": "v1",
                            "kind": "List",
                            "metadata": {
                                "resourceVersion": "1"
                            },
                            "items": [resource]
                        }
                    );
                    serde_json::to_vec(&list).unwrap()
                }
            }
        }
        ("PATCH", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources/foo") => {
            let parsed: Vec<serde_json::Value> =
                serde_json::from_slice(&request.into_body().collect_bytes().await.unwrap())
                    .unwrap();

            // Make sure the request is for stripping the finalizers
            assert_eq!(parsed.first().unwrap()["op"], "test");
            assert_eq!(parsed.last().unwrap()["op"], "remove");
            assert_eq!(parsed.last().unwrap()["path"], "/metadata/finalizers/0");

            // Remove from resource before serde
            resource.meta_mut().finalizers = None;
            serde_json::to_vec(&resource).unwrap()
        }
        _ => panic!("Unexpected API request {:?}", request),
    };

    send.send_response(Response::builder().body(Body::from(body)).unwrap());
}

pub async fn mock_ingest_resources(
    handle: &mut Handle<Request<Body>, Response<Body>>,
    assert_resources: Vec<FakeAPIResource>,
    num_created: &mut i32,
) {
    let (request, send) = handle.next_request().await.expect("Service not called");

    let body = match (
        request.method().as_str(),
        request.uri().path().to_string().as_str(),
    ) {
        ("GET", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources") => {
            match request.uri().query() {
                Some(q) if q.contains("watch") => vec![],
                _ => {
                    let list = serde_json::json!(
                        {
                            "apiVersion": "v1",
                            "kind": "List",
                            "metadata": {
                                "resourceVersion": "1"
                            },
                            "items": []
                        }
                    );
                    serde_json::to_vec(&list).unwrap()
                }
            }
        }
        ("GET", _) => {
            let id = request
                .uri()
                .path()
                .to_string()
                .split("/")
                .last()
                .unwrap()
                .split("-")
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            serde_json::to_vec(&assert_resources.get(id as usize).map(Clone::clone).unwrap())
                .unwrap()
        }
        ("POST", _) => {
            let parsed: FakeResource =
                serde_json::from_slice(&request.into_body().collect_bytes().await.unwrap())
                    .unwrap();

            assert_eq!(
                assert_resources
                    .get(parsed.spec.api_resource.id as usize)
                    .map(Clone::clone)
                    .unwrap(),
                parsed.spec.api_resource
            );
            assert_eq!(
                parsed
                    .metadata
                    .annotations
                    .as_ref()
                    .unwrap()
                    .get("databricks-operator/owner")
                    .unwrap(),
                "api"
            );

            *num_created += 1;
            serde_json::to_vec(&parsed).unwrap()
        }
        _ => panic!("Unexpected API request {:?}", request),
    };

    send.send_response(Response::builder().body(Body::from(body)).unwrap());
}
