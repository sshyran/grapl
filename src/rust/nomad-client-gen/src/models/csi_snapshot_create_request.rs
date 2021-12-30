/*
 * Nomad
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.1.4
 * Contact: support@hashicorp.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CsiSnapshotCreateRequest {
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "SecretID", skip_serializing_if = "Option::is_none")]
    pub secret_id: Option<String>,
    #[serde(rename = "Snapshots", skip_serializing_if = "Option::is_none")]
    pub snapshots: Option<Vec<crate::models::CsiSnapshot>>,
}

impl CsiSnapshotCreateRequest {
    pub fn new() -> CsiSnapshotCreateRequest {
        CsiSnapshotCreateRequest {
            namespace: None,
            region: None,
            secret_id: None,
            snapshots: None,
        }
    }
}
