use databricks_rust_jobs::models::job::Job;
use k8s_openapi::serde::{Deserialize, Serialize};
use kube::{CustomResource, CustomResourceExt};
use schemars::JsonSchema;

#[derive(Clone, CustomResource, Debug, Default, Deserialize, PartialEq, Serialize, JsonSchema)]
#[kube(
    group = "com.dstancu",
    version = "v1",
    kind = "DatabricksJob",
    namespaced
)]
pub struct DatabricksJobSpec {
    pub job: Job,
}
