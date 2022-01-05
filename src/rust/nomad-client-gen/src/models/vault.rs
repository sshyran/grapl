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
pub struct Vault {
    #[serde(rename = "ChangeMode", skip_serializing_if = "Option::is_none")]
    pub change_mode: Option<String>,
    #[serde(rename = "ChangeSignal", skip_serializing_if = "Option::is_none")]
    pub change_signal: Option<String>,
    #[serde(rename = "Env", skip_serializing_if = "Option::is_none")]
    pub env: Option<bool>,
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Policies", skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,
}

impl Vault {
    pub fn new() -> Vault {
        Vault {
            change_mode: None,
            change_signal: None,
            env: None,
            namespace: None,
            policies: None,
        }
    }
}
