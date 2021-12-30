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
pub struct Constraint {
    #[serde(rename = "LTarget", skip_serializing_if = "Option::is_none")]
    pub l_target: Option<String>,
    #[serde(rename = "Operand", skip_serializing_if = "Option::is_none")]
    pub operand: Option<String>,
    #[serde(rename = "RTarget", skip_serializing_if = "Option::is_none")]
    pub r_target: Option<String>,
}

impl Constraint {
    pub fn new() -> Constraint {
        Constraint {
            l_target: None,
            operand: None,
            r_target: None,
        }
    }
}
