mod databricks_job;

use kube::CustomResourceExt;
use serde_yaml::to_string;

fn main() {
    print!(
        "---\n{}",
        to_string(&crate::databricks_job::DatabricksJob::crd()).unwrap()
    );
}
