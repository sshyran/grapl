/*
 * Nomad
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.1.4
 * Contact: support@hashicorp.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{
    configuration,
    Error,
};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`create_quota_spec`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateQuotaSpecError {
    Status400(),
    Status403(),
    Status405(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_quota_spec`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteQuotaSpecError {
    Status400(),
    Status403(),
    Status405(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_quota_spec`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetQuotaSpecError {
    Status400(),
    Status403(),
    Status405(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_quotas`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetQuotasError {
    Status400(),
    Status403(),
    Status405(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_quota_spec`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostQuotaSpecError {
    Status400(),
    Status403(),
    Status405(),
    Status500(),
    UnknownValue(serde_json::Value),
}

pub async fn create_quota_spec(
    configuration: &configuration::Configuration,
    quota_spec: crate::models::QuotaSpec,
    region: Option<&str>,
    namespace: Option<&str>,
    x_nomad_token: Option<&str>,
    idempotency_token: Option<&str>,
) -> Result<(), Error<CreateQuotaSpecError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/quota", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = region {
        local_var_req_builder =
            local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = namespace {
        local_var_req_builder =
            local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = idempotency_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("idempotency_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_nomad_token {
        local_var_req_builder =
            local_var_req_builder.header("X-Nomad-Token", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Nomad-Token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&quota_spec);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CreateQuotaSpecError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_quota_spec(
    configuration: &configuration::Configuration,
    spec_name: &str,
    region: Option<&str>,
    namespace: Option<&str>,
    x_nomad_token: Option<&str>,
    idempotency_token: Option<&str>,
) -> Result<(), Error<DeleteQuotaSpecError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/quota/{specName}",
        local_var_configuration.base_path,
        specName = crate::apis::urlencode(spec_name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = region {
        local_var_req_builder =
            local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = namespace {
        local_var_req_builder =
            local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = idempotency_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("idempotency_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_nomad_token {
        local_var_req_builder =
            local_var_req_builder.header("X-Nomad-Token", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Nomad-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteQuotaSpecError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_quota_spec(
    configuration: &configuration::Configuration,
    spec_name: &str,
    region: Option<&str>,
    namespace: Option<&str>,
    index: Option<i32>,
    wait: Option<&str>,
    stale: Option<&str>,
    prefix: Option<&str>,
    x_nomad_token: Option<&str>,
    per_page: Option<i32>,
    next_token: Option<&str>,
) -> Result<crate::models::QuotaSpec, Error<GetQuotaSpecError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/quota/{specName}",
        local_var_configuration.base_path,
        specName = crate::apis::urlencode(spec_name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = region {
        local_var_req_builder =
            local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = namespace {
        local_var_req_builder =
            local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = wait {
        local_var_req_builder =
            local_var_req_builder.query(&[("wait", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = stale {
        local_var_req_builder =
            local_var_req_builder.query(&[("stale", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = prefix {
        local_var_req_builder =
            local_var_req_builder.query(&[("prefix", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder =
            local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = next_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("next_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = index {
        local_var_req_builder =
            local_var_req_builder.header("index", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_nomad_token {
        local_var_req_builder =
            local_var_req_builder.header("X-Nomad-Token", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Nomad-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetQuotaSpecError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_quotas(
    configuration: &configuration::Configuration,
    region: Option<&str>,
    namespace: Option<&str>,
    index: Option<i32>,
    wait: Option<&str>,
    stale: Option<&str>,
    prefix: Option<&str>,
    x_nomad_token: Option<&str>,
    per_page: Option<i32>,
    next_token: Option<&str>,
) -> Result<Vec<serde_json::Value>, Error<GetQuotasError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/quotas", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = region {
        local_var_req_builder =
            local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = namespace {
        local_var_req_builder =
            local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = wait {
        local_var_req_builder =
            local_var_req_builder.query(&[("wait", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = stale {
        local_var_req_builder =
            local_var_req_builder.query(&[("stale", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = prefix {
        local_var_req_builder =
            local_var_req_builder.query(&[("prefix", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder =
            local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = next_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("next_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = index {
        local_var_req_builder =
            local_var_req_builder.header("index", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_nomad_token {
        local_var_req_builder =
            local_var_req_builder.header("X-Nomad-Token", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Nomad-Token", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetQuotasError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn post_quota_spec(
    configuration: &configuration::Configuration,
    spec_name: &str,
    quota_spec: crate::models::QuotaSpec,
    region: Option<&str>,
    namespace: Option<&str>,
    x_nomad_token: Option<&str>,
    idempotency_token: Option<&str>,
) -> Result<(), Error<PostQuotaSpecError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/quota/{specName}",
        local_var_configuration.base_path,
        specName = crate::apis::urlencode(spec_name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = region {
        local_var_req_builder =
            local_var_req_builder.query(&[("region", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = namespace {
        local_var_req_builder =
            local_var_req_builder.query(&[("namespace", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = idempotency_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("idempotency_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_nomad_token {
        local_var_req_builder =
            local_var_req_builder.header("X-Nomad-Token", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Nomad-Token", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&quota_spec);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<PostQuotaSpecError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
