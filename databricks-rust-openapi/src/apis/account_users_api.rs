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


/// struct for typed errors of method [`account_userscreate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountUserscreateError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`account_usersdelete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountUsersdeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`account_usersget`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountUsersgetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`account_userslist`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountUserslistError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`account_userspatch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountUserspatchError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`account_usersupdate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountUsersupdateError {
    UnknownValue(serde_json::Value),
}


/// Creates a new user in the Databricks account. This new user will also be added to the Databricks account.
pub async fn account_userscreate(configuration: &configuration::Configuration, account_id: serde_json::Value, iam_user: crate::models::IamUser) -> Result<crate::models::IamUser, Error<AccountUserscreateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.0/accounts/{account_id}/scim/v2/Users", local_var_configuration.base_path, account_id=account_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&iam_user);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AccountUserscreateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a user. Deleting a user from a Databricks account also removes objects associated with the user.
pub async fn account_usersdelete(configuration: &configuration::Configuration, account_id: serde_json::Value, id: serde_json::Value) -> Result<serde_json::Value, Error<AccountUsersdeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.0/accounts/{account_id}/scim/v2/Users/{id}", local_var_configuration.base_path, account_id=account_id, id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        let local_var_entity: Option<AccountUsersdeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets information for a specific user in Databricks account.
pub async fn account_usersget(configuration: &configuration::Configuration, account_id: serde_json::Value, id: serde_json::Value) -> Result<crate::models::IamUser, Error<AccountUsersgetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.0/accounts/{account_id}/scim/v2/Users/{id}", local_var_configuration.base_path, account_id=account_id, id=id);
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
        let local_var_entity: Option<AccountUsersgetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets details for all the users associated with a Databricks account.
pub async fn account_userslist(configuration: &configuration::Configuration, account_id: serde_json::Value, filter: Option<&str>, attributes: Option<&str>, excluded_attributes: Option<&str>, start_index: Option<i32>, count: Option<i32>, sort_by: Option<&str>, sort_order: Option<&str>) -> Result<crate::models::IamListUsersResponse, Error<AccountUserslistError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.0/accounts/{account_id}/scim/v2/Users", local_var_configuration.base_path, account_id=account_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder = local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = attributes {
        local_var_req_builder = local_var_req_builder.query(&[("attributes", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = excluded_attributes {
        local_var_req_builder = local_var_req_builder.query(&[("excludedAttributes", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_index {
        local_var_req_builder = local_var_req_builder.query(&[("startIndex", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = count {
        local_var_req_builder = local_var_req_builder.query(&[("count", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort_by {
        local_var_req_builder = local_var_req_builder.query(&[("sortBy", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort_order {
        local_var_req_builder = local_var_req_builder.query(&[("sortOrder", &local_var_str.to_string())]);
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
        let local_var_entity: Option<AccountUserslistError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Partially updates a user resource by applying the supplied operations on specific user attributes.
pub async fn account_userspatch(configuration: &configuration::Configuration, account_id: serde_json::Value, id: serde_json::Value, iam_partial_update: crate::models::IamPartialUpdate) -> Result<serde_json::Value, Error<AccountUserspatchError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.0/accounts/{account_id}/scim/v2/Users/{id}", local_var_configuration.base_path, account_id=account_id, id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&iam_partial_update);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AccountUserspatchError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Replaces a user's information with the data supplied in request.
pub async fn account_usersupdate(configuration: &configuration::Configuration, account_id: serde_json::Value, id: serde_json::Value, iam_user: crate::models::IamUser) -> Result<serde_json::Value, Error<AccountUsersupdateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.0/accounts/{account_id}/scim/v2/Users/{id}", local_var_configuration.base_path, account_id=account_id, id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&iam_user);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AccountUsersupdateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

