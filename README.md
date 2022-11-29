---
description: A Kubernetes operator for Databricks
coverY: 0
---

# 🦀 databricks-kube-operator

[![Rust](https://github.com/mach-kernel/databricks-kube-operator/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/mach-kernel/databricks-kube-operator/actions/workflows/rust.yml)
[![FOSSA Status](https://app.fossa.com/api/projects/custom%2B34302%2Fgithub.com%2Fmach-kernel%2Fdatabricks-kube-operator.svg?type=shield)](https://app.fossa.com/projects/custom%2B34302%2Fgithub.com%2Fmach-kernel%2Fdatabricks-kube-operator?ref=badge_shield)

A [kube-rs](https://kube.rs/) operator to enable GitOps style management of Databricks resources. It supports the following APIs:

| API                 | CRD           |
| ------------------- | ------------- |
| Jobs 2.1            | DatabricksJob |
| Git Credentials 2.0 | GitCredential |
| Repos 2.0           | Repo          |

WIP and experimental! See the GitHub project board for the roadmap. Contributions and feedback are welcome!

[Read the docs](https://databricks-kube-operator.gitbook.io/doc)

## Quick Start

Looking for a more in-depth example? Read the [tutorial](tutorial.md).

### Installation

Add the Helm repository and install the chart:

```bash
helm repo add mach https://mach-kernel.github.io/databricks-kube-operator
helm install databricks-kube-operator mach/databricks-kube-operator
```

Create a config map in the same namespace as the operator. To override the configmap name, `--set configMapName=my-custom-name`:

```bash
cat <<EOF | kubectl apply -f -
apiVersion: v1
kind: ConfigMap
metadata:
  name: databricks-kube-operator
data:
  api_secret_name: databricks-api-secret
EOF
```

Create a secret with your API URL and credentials:

```bash
cat <<EOF | kubectl apply -f -
apiVersion: v1
data:
  access_token: $(echo -n 'shhhh' | base64)
  databricks_url: $(echo -n 'https://my-tenant.cloud.databricks.com/api' | base64)
kind: Secret
metadata:
  name: databricks-api-secret
type: Opaque
EOF
```

### Usage

See the examples directory for samples of Databricks CRDs. Resources that are created via Kubernetes are owned by the operator: your checked-in manifests are the source of truth. It will not sync anything other than status back from the API, and overwrite changes made by users from the Databricks webapp.

You may provide the `databricks-operator/owner` annotation as shown below (to be explicit). However, all resources created in Kube first (i.e. no associated API object found) are assumed to be owned by the operator.

```yaml
apiVersion: com.dstancu.databricks/v1
kind: DatabricksJob
metadata:
  name: my-super-cool-job
  namespace: default
  annotations:
    databricks-operator/owner: operator
```

By default, databricks-kube-operator will also sync existing API resources from Databricks into Kubernetes (goal: surface status). Resources owned by the API are tagged as such with an annotation on ingest:

```yaml
apiVersion: com.dstancu.databricks/v1
kind: DatabricksJob
metadata:
  annotations:
    databricks-operator/owner: api
  creationTimestamp: "2022-11-04T21:46:12Z"
  generation: 1
  name: hello-world
  ...
```

Look at jobs (allowed to be viewed by the operator's access token):

```bash
$ kubectl get databricksjobs
NAME                                 STATUS
contoso-ingest-qa                      RUNNING
contoso-ingest-staging                 INTERNAL_ERROR
contoso-stats-qa                       TERMINATED
contoso-stats-staging                  NO_RUNS

$ kubectl describe databricksjob contoso-ingest-qa
...
```

A job's status key surfaces API information about the latest [run](https://docs.databricks.com/dev-tools/api/latest/jobs.html#operation/JobsRunsList). The status is polled every 60s:

```bash
$ kubectl get databricksjob contoso-ingest-staging -ojson | jq .status
{
  "latest_run_state": {
    "life_cycle_state": "INTERNAL_ERROR",
    "result_state": "FAILED",
    "state_message": "Task contoso-ingest-staging failed. This caused all downstream tasks to get skipped.",
    "user_cancelled_or_timedout": false
  }
}
```

## Developers

Begin by creating the configmap as per the Helm instructions.

Generate and install the CRDs by running the `crd_gen` bin target:

```bash
cargo run --bin crd_gen | kubectl apply -f -
```

The quickest way to test the operator is with a working [minikube](https://minikube.sigs.k8s.io/docs/start/) cluster:

```bash
minikube start
minikube tunnel &
```

```bash
export RUST_LOG=databricks_kube
cargo run
[2022-11-02T18:56:25Z INFO  databricks_kube] boot! (build: df7e26b-modified)
[2022-11-02T18:56:25Z INFO  databricks_kube::context] Waiting for CRD: databricksjobs.com.dstancu.databricks
[2022-11-02T18:56:25Z INFO  databricks_kube::context] Waiting for CRD: gitcredentials.com.dstancu.databricks
[2022-11-02T18:56:25Z INFO  databricks_kube::context] Waiting for settings in config map: databricks-kube-operator
[2022-11-02T18:56:25Z INFO  databricks_kube::context] Found config map
[2022-11-02T18:56:25Z INFO  databricks_kube::traits::synced_api_resource] Looking for uningested GitCredential(s)
[2022-11-02T18:56:25Z INFO  databricks_kube::traits::synced_api_resource] Looking for uningested DatabricksJob(s)
```

### Generating API Clients

The client is generated by `openapi-generator` and then lightly postprocessed so we get models that derive [`JsonSchema`](https://github.com/GREsau/schemars#basic-usage) and fix some bugs.

TODO: Fork or fix generator/template issues instead of sed.

```bash
# Hey!! This uses GNU sed
# brew install gnu-sed

# Jobs API
openapi-generator generate -g rust -i openapi/jobs-2.1-aws.yaml -c openapi/config-jobs.yaml -o dbr_jobs

# Derive JsonSchema for all models and add schemars as dep
gsed -i -e 's/derive(Clone/derive(JsonSchema, Clone/' dbr_jobs/src/models/*
gsed -i -e 's/\/\*/use schemars::JsonSchema;\n\/\*/' dbr_jobs/src/models/*
gsed -r -i -e 's/(\[dependencies\])/\1\nschemars = "0.8.11"/' dbr_jobs/Cargo.toml

# Missing import?
gsed -r -i -e 's/(use reqwest;)/\1\nuse crate::models::ViewsToExport;/' dbr_jobs/src/apis/default_api.rs

# Git Credentials API
openapi-generator generate -g rust -i openapi/gitcredentials-2.0-aws.yaml -c openapi/config-git.yaml -o dbr_git_creds

# Derive JsonSchema for all models and add schemars as dep
gsed -i -e 's/derive(Clone/derive(JsonSchema, Clone/' dbr_git_creds/src/models/*
gsed -i -e 's/\/\*/use schemars::JsonSchema;\n\/\*/' dbr_git_creds/src/models/*
gsed -r -i -e 's/(\[dependencies\])/\1\nschemars = "0.8.11"/' dbr_git_creds/Cargo.toml

# Repos API
openapi-generator generate -g rust -i openapi/repos-2.0-aws.yaml -c openapi/config-repos.yaml -o dbr_repo

# Derive JsonSchema for all models and add schemars as dep
gsed -i -e 's/derive(Clone/derive(JsonSchema, Clone/' dbr_repo/src/models/*
gsed -i -e 's/\/\*/use schemars::JsonSchema;\n\/\*/' dbr_repo/src/models/*
gsed -r -i -e 's/(\[dependencies\])/\1\nschemars = "0.8.11"/' dbr_repo/Cargo.toml
```

### Expand CRD macros

Deriving `CustomResource` uses macros to generate another struct. For this example, the output struct name would be `DatabricksJob`:

```rust
#[derive(Clone, CustomResource, Debug, Default, Deserialize, PartialEq, Serialize, JsonSchema)]
#[kube(
    group = "com.dstancu.databricks",
    version = "v1",
    kind = "DatabricksJob",
    derive = "Default",
    namespaced
)]
pub struct DatabricksJobSpec {
    pub job: Job,
}
```

`rust-analyzer` shows squiggles when you `use crds::databricks_job::DatabricksJob`, but one may want to look inside. To see what is generated with [cargo-expand](https://github.com/dtolnay/cargo-expand):

```bash
rustup default nightly
cargo expand --bin databricks_kube
```

### Adding a new CRD

Want to add support for a new API? Provided it has an OpenAPI definition, these are the steps. Look for existing examples in the codebase:

* Download API definition into `openapi/` and make a [Rust generator configuration](https://openapi-generator.tech/docs/generators/rust/) (feel free to copy the others and change name)
* Generate the SDK, add it to the Cargo workspace and dependencies for `databricks-kube/`
* Implement `RestConfig<TSDKConfig>` for your new client
* Implement `From<TSDKAPIError<E>>` for `DatabricksKubeError`
* Define the new CRD Spec type ([follow kube-rs tutorial](https://kube.rs/getting-started/))
* `impl RemoteAPIResource<TAPIResource> for MyNewCRD`
* `impl StatusAPIResource<TStatusType> for MyNewCRD` and [specify `TStatusType` in your CRD](https://github.com/kube-rs/kube/blob/main/examples/crd_derive.rs#L20)
* Add the new resource to the context ensure CRDs condition
* Add the new resource to `crdgen.rs`

### Running tests

Tests must be run with a single thread since we use a stateful singleton to 'mock' the state of a remote API. Eventually it would be nice to have integration tests targetting Databricks.

```bash
$ cargo test -- --test-threads=1
```

## License

[![FOSSA Status](https://app.fossa.com/api/projects/custom%2B34302%2Fgithub.com%2Fmach-kernel%2Fdatabricks-kube-operator.svg?type=large)](https://app.fossa.com/projects/custom%2B34302%2Fgithub.com%2Fmach-kernel%2Fdatabricks-kube-operator?ref=badge_large)