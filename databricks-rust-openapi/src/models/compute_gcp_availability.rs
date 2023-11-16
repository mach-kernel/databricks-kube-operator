use schemars::JsonSchema;
/*
 * Databricks Accounts and Workspace REST API on ALL
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ComputeGcpAvailability : This field determines whether the instance pool will contain preemptible VMs, on-demand VMs, or preemptible VMs with a fallback to on-demand VMs if the former is unavailable.

/// This field determines whether the instance pool will contain preemptible VMs, on-demand VMs, or preemptible VMs with a fallback to on-demand VMs if the former is unavailable.
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ComputeGcpAvailability {
    #[serde(rename = "PREEMPTIBLE_GCP")]
    PreemptibleGcp,
    #[serde(rename = "ON_DEMAND_GCP")]
    OnDemandGcp,
    #[serde(rename = "PREEMPTIBLE_WITH_FALLBACK_GCP")]
    PreemptibleWithFallbackGcp,

}

impl ToString for ComputeGcpAvailability {
    fn to_string(&self) -> String {
        match self {
            Self::PreemptibleGcp => String::from("PREEMPTIBLE_GCP"),
            Self::OnDemandGcp => String::from("ON_DEMAND_GCP"),
            Self::PreemptibleWithFallbackGcp => String::from("PREEMPTIBLE_WITH_FALLBACK_GCP"),
        }
    }
}

impl Default for ComputeGcpAvailability {
    fn default() -> ComputeGcpAvailability {
        Self::PreemptibleGcp
    }
}




