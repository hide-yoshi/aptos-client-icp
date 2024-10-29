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

/// StateCheckpointTransaction : A state checkpoint transaction
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StateCheckpointTransaction {
    /// A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatibility with languages such as JavaScript that do not parse u64s in JSON natively. 
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "state_change_hash")]
    pub state_change_hash: String,
    #[serde(rename = "event_root_hash")]
    pub event_root_hash: String,
    #[serde(rename = "state_checkpoint_hash", skip_serializing_if = "Option::is_none")]
    pub state_checkpoint_hash: Option<String>,
    /// A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatibility with languages such as JavaScript that do not parse u64s in JSON natively. 
    #[serde(rename = "gas_used")]
    pub gas_used: String,
    /// Whether the transaction was successful
    #[serde(rename = "success")]
    pub success: bool,
    /// The VM status of the transaction, can tell useful information in a failure
    #[serde(rename = "vm_status")]
    pub vm_status: String,
    #[serde(rename = "accumulator_root_hash")]
    pub accumulator_root_hash: String,
    /// Final state of resources changed by the transaction
    #[serde(rename = "changes")]
    pub changes: Vec<models::WriteSetChange>,
    /// A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatibility with languages such as JavaScript that do not parse u64s in JSON natively. 
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}

impl StateCheckpointTransaction {
    /// A state checkpoint transaction
    pub fn new(version: String, hash: String, state_change_hash: String, event_root_hash: String, gas_used: String, success: bool, vm_status: String, accumulator_root_hash: String, changes: Vec<models::WriteSetChange>, timestamp: String) -> StateCheckpointTransaction {
        StateCheckpointTransaction {
            version,
            hash,
            state_change_hash,
            event_root_hash,
            state_checkpoint_hash: None,
            gas_used,
            success,
            vm_status,
            accumulator_root_hash,
            changes,
            timestamp,
        }
    }
}

