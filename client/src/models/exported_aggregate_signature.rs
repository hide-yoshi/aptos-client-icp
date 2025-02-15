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

/// ExportedAggregateSignature : A more API-friendly representation of the on-chain `aptos_types::aggregate_signature::AggregateSignature`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportedAggregateSignature {
    #[serde(rename = "signer_indices")]
    pub signer_indices: Vec<i32>,
    /// All bytes (Vec<u8>) data is represented as hex-encoded string prefixed with `0x` and fulfilled with two hex digits per byte.  Unlike the `Address` type, HexEncodedBytes will not trim any zeros. 
    #[serde(rename = "sig", skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
}

impl ExportedAggregateSignature {
    /// A more API-friendly representation of the on-chain `aptos_types::aggregate_signature::AggregateSignature`.
    pub fn new(signer_indices: Vec<i32>) -> ExportedAggregateSignature {
        ExportedAggregateSignature {
            signer_indices,
            sig: None,
        }
    }
}

