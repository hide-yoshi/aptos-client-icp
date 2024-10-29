/*
 * Aptos Node API
 *
 * The Aptos Node API is a RESTful API for client applications to interact with the Aptos blockchain.
 *
 * The version of the OpenAPI document: 1.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// GenesisPayload : The writeset payload of the Genesis transaction
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GenesisPayload {
    #[serde(rename="write_set_payload")]
    WriteSetPayload(Box<models::GenesisPayloadWriteSetPayload>),
}

impl Default for GenesisPayload {
    fn default() -> Self {
        Self::WriteSetPayload(Default::default())
    }
}


