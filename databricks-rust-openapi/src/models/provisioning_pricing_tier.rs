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

/// ProvisioningPricingTier : The pricing tier of the workspace. For pricing tier information, see [AWS Pricing](https://Databrickscom/product/aws-pricing). 

/// The pricing tier of the workspace. For pricing tier information, see [AWS Pricing](https://Databrickscom/product/aws-pricing). 
#[derive(JsonSchema, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProvisioningPricingTier {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "COMMUNITY_EDITION")]
    CommunityEdition,
    #[serde(rename = "STANDARD")]
    Standard,
    #[serde(rename = "PREMIUM")]
    Premium,
    #[serde(rename = "ENTERPRISE")]
    Enterprise,
    #[serde(rename = "DEDICATED")]
    Dedicated,

}

impl ToString for ProvisioningPricingTier {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => String::from("UNKNOWN"),
            Self::CommunityEdition => String::from("COMMUNITY_EDITION"),
            Self::Standard => String::from("STANDARD"),
            Self::Premium => String::from("PREMIUM"),
            Self::Enterprise => String::from("ENTERPRISE"),
            Self::Dedicated => String::from("DEDICATED"),
        }
    }
}

impl Default for ProvisioningPricingTier {
    fn default() -> ProvisioningPricingTier {
        Self::Unknown
    }
}




