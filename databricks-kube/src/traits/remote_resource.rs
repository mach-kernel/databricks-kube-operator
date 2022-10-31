use crate::{crds::databricks_job::DatabricksJobSpec, error::DatabricksKubeError};
use async_stream::try_stream;
use databricks_rust_jobs::{
    apis::{configuration::Configuration, default_api},
    models::{Job, JobsList200Response},
};
use futures::StreamExt;
use futures::{FutureExt, Stream};
use std::pin::Pin;

pub trait RemoteResource<TAPIType> {
    fn list_all(
        config: Configuration,
    ) -> Pin<Box<dyn Stream<Item = Result<TAPIType, DatabricksKubeError>> + Send>>;
}
