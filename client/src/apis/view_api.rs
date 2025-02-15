/*
 * Aptos Node API
 *
 * The Aptos Node API is a RESTful API for client applications to interact with the Aptos blockchain.
 *
 * The version of the OpenAPI document: 1.2.0
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`view`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ViewError {
    Status400(models::AptosError),
    Status403(models::AptosError),
    Status404(models::AptosError),
    Status410(models::AptosError),
    Status500(models::AptosError),
    Status503(models::AptosError),
    UnknownValue(serde_json::Value),
}

/// Execute the Move function with the given parameters and return its execution result.  The Aptos nodes prune account state history, via a configurable time window. If the requested ledger version has been pruned, the server responds with a 410.
pub async fn view(
    configuration: &configuration::Configuration,
    view_request: models::ViewRequest,
    ledger_version: Option<&str>,
) -> Result<Vec<models::MoveValue>, Error<ViewError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/view", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(crate::http::HttpMethod::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = ledger_version {
        local_var_req_builder =
            local_var_req_builder.query(&[("ledger_version", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header("User-Agent".to_string(), local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&view_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ViewError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
