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


/// struct for typed errors of method [`account_access_control_proxyget_assignable_roles_for_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountAccessControlProxygetAssignableRolesForResourceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`account_access_control_proxyget_rule_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountAccessControlProxygetRuleSetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`account_access_control_proxyupdate_rule_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountAccessControlProxyupdateRuleSetError {
    UnknownValue(serde_json::Value),
}


/// Gets all the roles that can be granted on an account-level resource. A role is grantable if the rule set on the resource can contain an access rule of the role. 
pub async fn account_access_control_proxyget_assignable_roles_for_resource(configuration: &configuration::Configuration, resource: serde_json::Value) -> Result<crate::models::IamGetAssignableRolesForResourceResponse, Error<AccountAccessControlProxygetAssignableRolesForResourceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.0/preview/accounts/access-control/assignable-roles", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("resource", &resource.to_string())]);
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
        let local_var_entity: Option<AccountAccessControlProxygetAssignableRolesForResourceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a rule set by its name. A rule set is always attached to a resource and contains a list of access rules on the said resource. Currently only a default rule set for each resource is supported. 
pub async fn account_access_control_proxyget_rule_set(configuration: &configuration::Configuration, name: serde_json::Value, etag: serde_json::Value) -> Result<crate::models::IamRuleSetResponse, Error<AccountAccessControlProxygetRuleSetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.0/preview/accounts/access-control/rule-sets", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("name", &name.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("etag", &etag.to_string())]);
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
        let local_var_entity: Option<AccountAccessControlProxygetRuleSetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Replace the rules of a rule set. First, use a GET rule set request to read the current version of the rule set before modifying it. This pattern helps prevent conflicts between concurrent updates. 
pub async fn account_access_control_proxyupdate_rule_set(configuration: &configuration::Configuration, iam_update_rule_set_request: crate::models::IamUpdateRuleSetRequest) -> Result<crate::models::IamRuleSetResponse, Error<AccountAccessControlProxyupdateRuleSetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/2.0/preview/accounts/access-control/rule-sets", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&iam_update_rule_set_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AccountAccessControlProxyupdateRuleSetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

