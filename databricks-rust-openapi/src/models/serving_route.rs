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
pub struct ServingRoute {
    /// The name of the served model this route configures traffic for.
    #[serde(rename = "served_model_name")]
    pub served_model_name: String,
    /// The percentage of endpoint traffic to send to this route. It must be an integer between 0 and 100 inclusive.
    #[serde(rename = "traffic_percentage")]
    pub traffic_percentage: i32,
}

impl ServingRoute {
    pub fn new(served_model_name: String, traffic_percentage: i32) -> ServingRoute {
        ServingRoute {
            served_model_name,
            traffic_percentage,
        }
    }
}


