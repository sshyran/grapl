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
pub struct ReschedulePolicy {
    #[serde(rename = "Attempts", skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i32>,
    #[serde(rename = "Delay", skip_serializing_if = "Option::is_none")]
    pub delay: Option<i64>,
    #[serde(rename = "DelayFunction", skip_serializing_if = "Option::is_none")]
    pub delay_function: Option<String>,
    #[serde(rename = "Interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[serde(rename = "MaxDelay", skip_serializing_if = "Option::is_none")]
    pub max_delay: Option<i64>,
    #[serde(rename = "Unlimited", skip_serializing_if = "Option::is_none")]
    pub unlimited: Option<bool>,
}

impl ReschedulePolicy {
    pub fn new() -> ReschedulePolicy {
        ReschedulePolicy {
            attempts: None,
            delay: None,
            delay_function: None,
            interval: None,
            max_delay: None,
            unlimited: None,
        }
    }
}
