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




#[derive(JsonSchema, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputePolicy {
    /// Creator user name. The field won't be included in the response if the user has already been deleted.
    #[serde(rename = "creator_user_name", skip_serializing_if = "Option::is_none")]
    pub creator_user_name: Option<String>,
    /// If true, policy is a default policy created and managed by <Databricks>. Default policies cannot be deleted, and their policy families cannot be changed.
    #[serde(rename = "is_default", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Max number of clusters per user that can be active using this policy. If not present, there is no max limit.
    #[serde(rename = "max_clusters_per_user", skip_serializing_if = "Option::is_none")]
    pub max_clusters_per_user: Option<i64>,
    /// Canonical unique identifier for the Cluster Policy.
    #[serde(rename = "policy_id", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// Cluster Policy name requested by the user. This has to be unique. Length must be between 1 and 100 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Policy definition document expressed in Databricks Cluster Policy Definition Language.
    #[serde(rename = "definition", skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    /// Policy definition JSON document expressed in Databricks Policy Definition Language. The JSON document must be passed as a string and cannot be embedded in the requests.  You can use this to customize the policy definition inherited from the policy family. Policy rules specified here are merged into the inherited policy definition. 
    #[serde(rename = "policy_family_definition_overrides", skip_serializing_if = "Option::is_none")]
    pub policy_family_definition_overrides: Option<String>,
    /// ID of the policy family.
    #[serde(rename = "policy_family_id", skip_serializing_if = "Option::is_none")]
    pub policy_family_id: Option<String>,
    /// Creation time. The timestamp (in millisecond) when this Cluster Policy was created.
    #[serde(rename = "created_at_timestamp", skip_serializing_if = "Option::is_none")]
    pub created_at_timestamp: Option<i64>,
    /// Additional human-readable description of the cluster policy.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ComputePolicy {
    pub fn new() -> ComputePolicy {
        ComputePolicy {
            creator_user_name: None,
            is_default: None,
            max_clusters_per_user: None,
            policy_id: None,
            name: None,
            definition: None,
            policy_family_definition_overrides: None,
            policy_family_id: None,
            created_at_timestamp: None,
            description: None,
        }
    }
}


