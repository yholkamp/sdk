/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.5
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Session {
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "authenticated_at")]
    pub authenticated_at: String,
    #[serde(rename = "expires_at")]
    pub expires_at: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "identity")]
    pub identity: Box<crate::models::Identity>,
    #[serde(rename = "issued_at")]
    pub issued_at: String,
}

impl Session {
    pub fn new(authenticated_at: String, expires_at: String, id: String, identity: crate::models::Identity, issued_at: String) -> Session {
        Session {
            active: None,
            authenticated_at,
            expires_at,
            id,
            identity: Box::new(identity),
            issued_at,
        }
    }
}


