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
pub struct ComputeGlobalInitScriptUpdateRequest {
    /// Specifies whether the script is enabled. The script runs only if enabled.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The name of the script
    #[serde(rename = "name")]
    pub name: String,
    /// The position of a script, where 0 represents the first script to run, 1 is the second script to run, in ascending order. To move the script to run first, set its position to 0.  To move the script to the end, set its position to any value greater or equal to the position of the last script. Example, three existing scripts with positions 0, 1, and 2. Any position value of 2 or greater puts the script in the last position (2).  If an explicit position value conflicts with an existing script, your request succeeds, but the original script at that position and all later scripts have their positions incremented by 1. 
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// The Base64-encoded content of the script.
    #[serde(rename = "script")]
    pub script: String,
}

impl ComputeGlobalInitScriptUpdateRequest {
    pub fn new(name: String, script: String) -> ComputeGlobalInitScriptUpdateRequest {
        ComputeGlobalInitScriptUpdateRequest {
            enabled: None,
            name,
            position: None,
            script,
        }
    }
}


