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
pub struct CatalogEffectivePermissionsList {
    #[serde(rename = "privilege_assignments", skip_serializing_if = "Option::is_none")]
    pub privilege_assignments: Option<Vec<crate::models::CatalogEffectivePrivilegeAssignment>>,
}

impl CatalogEffectivePermissionsList {
    pub fn new() -> CatalogEffectivePermissionsList {
        CatalogEffectivePermissionsList {
            privilege_assignments: None,
        }
    }
}


