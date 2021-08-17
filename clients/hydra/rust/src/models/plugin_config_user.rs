/*
 * ORY Hydra
 *
 * Welcome to the ORY Hydra HTTP API documentation. You will find documentation for all HTTP APIs here.
 *
 * The version of the OpenAPI document: v1.10.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PluginConfigUser : PluginConfigUser plugin config user



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginConfigUser {
    /// g ID
    #[serde(rename = "GID", skip_serializing_if = "Option::is_none")]
    pub GID: Option<i32>,
    /// UID
    #[serde(rename = "UID", skip_serializing_if = "Option::is_none")]
    pub UID: Option<i32>,
}

impl PluginConfigUser {
    /// PluginConfigUser plugin config user
    pub fn new() -> PluginConfigUser {
        PluginConfigUser {
            GID: None,
            UID: None,
        }
    }
}


