/*
 * ORY Hydra
 *
 * Welcome to the ORY Hydra HTTP API documentation. You will find documentation for all HTTP APIs here.
 *
 * The version of the OpenAPI document: v1.10.5
 * 
 * Generated by: https://openapi-generator.tech
 */

/// JsonError : Error responses are sent when an error (e.g. unauthorized, bad request, ...) occurred.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonError {
    /// Name is the error name.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Debug contains debug information. This is usually not available and has to be enabled.
    #[serde(rename = "error_debug", skip_serializing_if = "Option::is_none")]
    pub error_debug: Option<String>,
    /// Description contains further information on the nature of the error.
    #[serde(rename = "error_description", skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    /// Code represents the error status code (404, 403, 401, ...).
    #[serde(rename = "status_code", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

impl JsonError {
    /// Error responses are sent when an error (e.g. unauthorized, bad request, ...) occurred.
    pub fn new() -> JsonError {
        JsonError {
            error: None,
            error_debug: None,
            error_description: None,
            status_code: None,
        }
    }
}


