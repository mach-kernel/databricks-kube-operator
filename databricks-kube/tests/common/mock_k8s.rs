
async fn mock_get_fakeresources(handle: &mut Handle<Request<Body>, Response<Body>>) {
    let (request, send) = handle.next_request().await.expect("Service not called");

    let body = match (request.method().as_str(), request.path().to_string().as_str()) {
        ("GET", "/apis/com.dstancu.test/v1/namespaces/default/fakeresources") => {
            let resource: Vec<FakeResource> = serde_json::from_value(serde_json::json!([
                {
                    "apiVersion": "com.dstancu.test/v1",
                    "kind": "FakeResource",
                    "metadata": {
                        "name": "test",
                    },
                    "spec": {
                        "api_resource": {
                            "id": 1
                        }
                    }
                }
            ])).unwrap();

            serde_json::to_vec(&resource).unwrap()
        }
        _ => panic!("Unexpected API request {:?}", request),
    };

    send.send_response(Response::builder().body(Body::from(body)).unwrap());
}