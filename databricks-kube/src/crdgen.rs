mod context;
mod crds;
mod error;
mod traits;

use kube::CustomResourceExt;
use serde_yaml::to_string;

fn main() {
    print!(
        "---\n{}\n",
        to_string(&crate::crds::databricks_job::DatabricksJob::crd()).unwrap()
    );
    print!(
        "---\n{}\n",
        to_string(&crate::crds::git_credential::GitCredential::crd()).unwrap()
    );
    print!(
        "---\n{}\n",
        to_string(&crate::crds::repo::Repo::crd()).unwrap()
    );
}
