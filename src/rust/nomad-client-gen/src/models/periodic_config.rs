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
pub struct PeriodicConfig {
    #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "ProhibitOverlap", skip_serializing_if = "Option::is_none")]
    pub prohibit_overlap: Option<bool>,
    #[serde(rename = "Spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<String>,
    #[serde(rename = "SpecType", skip_serializing_if = "Option::is_none")]
    pub spec_type: Option<String>,
    #[serde(rename = "TimeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

impl PeriodicConfig {
    pub fn new() -> PeriodicConfig {
        PeriodicConfig {
            enabled: None,
            prohibit_overlap: None,
            spec: None,
            spec_type: None,
            time_zone: None,
        }
    }
}
