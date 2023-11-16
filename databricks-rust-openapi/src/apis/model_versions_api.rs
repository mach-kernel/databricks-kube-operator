/*
 * Databricks Accounts and Workspace REST API on ALL
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`model_versionsdelete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelVersionsdeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`model_versionsget`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelVersionsgetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`model_versionslist`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelVersionslistError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`model_versionsupdate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelVersionsupdateError {
    UnknownValue(serde_json::Value),
}


/// Deletes a model version from the specified registered model. Any aliases assigned to the model version will also be deleted.  The caller must be a metastore admin or an owner of the parent registered model. For the latter case, the caller must also be the owner or have the **USE_CATALOG** privilege on the parent catalog and the **USE_SCHEMA** privilege on the parent schema. 
pub async fn model_versionsdelete(configuration: &configuration::Configuration, full_name: &str, version: i32) -> Result<(), Error<ModelVersionsdeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.1/unity-catalog/models/{full_name}/versions/{version}", local_var_configuration.base_path, full_name=crate::apis::urlencode(full_name), version=version);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ModelVersionsdeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a model version.  The caller must be a metastore admin or an owner of (or have the **EXECUTE** privilege on) the parent registered model. For the latter case, the caller must also be the owner or have the **USE_CATALOG** privilege on the parent catalog and the **USE_SCHEMA** privilege on the parent schema. 
pub async fn model_versionsget(configuration: &configuration::Configuration, full_name: &str, version: i32) -> Result<crate::models::CatalogRegisteredModelInfo, Error<ModelVersionsgetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.1/unity-catalog/models/{full_name}/versions/{version}", local_var_configuration.base_path, full_name=crate::apis::urlencode(full_name), version=version);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ModelVersionsgetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List model versions. You can list model versions under a particular schema, or list all model versions in the current metastore.  The returned models are filtered based on the privileges of the calling user. For example, the metastore admin is able to list all the model versions. A regular user needs to be the owner or have the **EXECUTE** privilege on the parent registered model to recieve the model versions in the response. For the latter case, the caller must also be the owner or have the **USE_CATALOG** privilege on the parent catalog and the **USE_SCHEMA** privilege on the parent schema.  There is no guarantee of a specific ordering of the elements in the response. 
pub async fn model_versionslist(configuration: &configuration::Configuration, full_name: &str, max_results: Option<i32>, page_token: Option<&str>) -> Result<crate::models::CatalogListModelVersionsResponse, Error<ModelVersionslistError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.1/unity-catalog/models/{full_name}/versions", local_var_configuration.base_path, full_name=crate::apis::urlencode(full_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = max_results {
        local_var_req_builder = local_var_req_builder.query(&[("max_results", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder = local_var_req_builder.query(&[("page_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ModelVersionslistError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates the specified model version.  The caller must be a metastore admin or an owner of the parent registered model. For the latter case, the caller must also be the owner or have the **USE_CATALOG** privilege on the parent catalog and the **USE_SCHEMA** privilege on the parent schema.  Currently only the comment of the model version can be updated. 
pub async fn model_versionsupdate(configuration: &configuration::Configuration, full_name: &str, version: i32, catalog_update_model_version_request: Option<crate::models::CatalogUpdateModelVersionRequest>) -> Result<crate::models::CatalogModelVersionInfo, Error<ModelVersionsupdateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.1/unity-catalog/models/{full_name}/versions/{version}", local_var_configuration.base_path, full_name=crate::apis::urlencode(full_name), version=version);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&catalog_update_model_version_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ModelVersionsupdateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

