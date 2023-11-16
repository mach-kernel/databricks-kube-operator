use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

pub mod acl_permissions_api;
pub mod account_access_control_api;
pub mod account_access_control_proxy_api;
pub mod account_groups_api;
pub mod account_ip_access_lists_api;
pub mod account_metastore_assignments_api;
pub mod account_metastores_api;
pub mod account_service_principals_api;
pub mod account_storage_credentials_api;
pub mod account_users_api;
pub mod alerts_api;
pub mod artifact_allowlists_api;
pub mod billable_usage_download_api;
pub mod budgets_api;
pub mod catalogs_api;
pub mod clean_rooms_api;
pub mod cluster_policies_api;
pub mod clusters_api;
pub mod command_execution_api;
pub mod connections_api;
pub mod credential_configurations_api;
pub mod current_user_api;
pub mod dbfs_api;
pub mod dashboards_api;
pub mod data_sources_api;
pub mod experiments_api;
pub mod external_locations_api;
pub mod functions_api;
pub mod git_credentials_api;
pub mod global_init_scripts_api;
pub mod grants_api;
pub mod groups_api;
pub mod ip_access_lists_api;
pub mod instance_pools_api;
pub mod instance_profiles_api;
pub mod jobs_api;
pub mod key_configurations_api;
pub mod log_delivery_configurations_api;
pub mod managed_libraries_api;
pub mod metastores_api;
pub mod model_registry_api;
pub mod model_versions_api;
pub mod network_configurations_api;
pub mod network_policy_api;
pub mod o_auth_custom_app_integration_api;
pub mod o_auth_enrollment_api;
pub mod o_auth_published_app_integration_api;
pub mod permissions_api;
pub mod personal_compute_enablement_api;
pub mod pipelines_api;
pub mod policy_families_api;
pub mod private_access_settings_api;
pub mod providers_api;
pub mod queries_results_api;
pub mod query_history_api;
pub mod recipient_activation_api;
pub mod recipients_api;
pub mod registered_models_api;
pub mod repos_api;
pub mod sql_warehouses_api;
pub mod schemas_api;
pub mod secret_api;
pub mod securable_tags_api;
pub mod service_principal_secrets_api;
pub mod service_principals_api;
pub mod serving_endpoints_api;
pub mod shares_api;
pub mod statement_execution_api;
pub mod storage_configurations_api;
pub mod storage_credentials_api;
pub mod subentity_tags_api;
pub mod system_schemas_api;
pub mod table_constraints_api;
pub mod tables_api;
pub mod token_api;
pub mod token_management_api;
pub mod users_api;
pub mod vpc_endpoint_configurations_api;
pub mod volumes_api;
pub mod workspace_api;
pub mod workspace_assignment_api;
pub mod workspace_bindings_api;
pub mod workspace_conf_api;
pub mod workspaces_api;

pub mod configuration;
