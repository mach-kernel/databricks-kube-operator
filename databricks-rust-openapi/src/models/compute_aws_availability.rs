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

/// ComputeAwsAvailability : Availability type used for all subsequent nodes past the `first_on_demand` ones.  Note: If `first_on_demand` is zero, this availability type will be used for the entire cluster. 

/// Availability type used for all subsequent nodes past the `first_on_demand` ones.  Note: If `first_on_demand` is zero, this availability type will be used for the entire cluster. 
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ComputeAwsAvailability {
    #[serde(rename = "SPOT")]
    Spot,
    #[serde(rename = "ON_DEMAND")]
    OnDemand,
    #[serde(rename = "SPOT_WITH_FALLBACK")]
    SpotWithFallback,

}

impl ToString for ComputeAwsAvailability {
    fn to_string(&self) -> String {
        match self {
            Self::Spot => String::from("SPOT"),
            Self::OnDemand => String::from("ON_DEMAND"),
            Self::SpotWithFallback => String::from("SPOT_WITH_FALLBACK"),
        }
    }
}

impl Default for ComputeAwsAvailability {
    fn default() -> ComputeAwsAvailability {
        Self::Spot
    }
}




