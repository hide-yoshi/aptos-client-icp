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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidatorTransactionJwkUpdateTransaction {
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
    #[serde(rename = "events")]
    pub events: Vec<models::Event>,
    /// A string containing a 64-bit unsigned integer.  We represent u64 values as a string to ensure compatibility with languages such as JavaScript that do not parse u64s in JSON natively. 
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "quorum_certified_update")]
    pub quorum_certified_update: Box<models::ExportedQuorumCertifiedUpdate>,
    #[serde(rename = "validator_transaction_type")]
    pub validator_transaction_type: ValidatorTransactionType,
}

impl ValidatorTransactionJwkUpdateTransaction {
    pub fn new(version: String, hash: String, state_change_hash: String, event_root_hash: String, gas_used: String, success: bool, vm_status: String, accumulator_root_hash: String, changes: Vec<models::WriteSetChange>, events: Vec<models::Event>, timestamp: String, quorum_certified_update: models::ExportedQuorumCertifiedUpdate, validator_transaction_type: ValidatorTransactionType) -> ValidatorTransactionJwkUpdateTransaction {
        ValidatorTransactionJwkUpdateTransaction {
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
            events,
            timestamp,
            quorum_certified_update: Box::new(quorum_certified_update),
            validator_transaction_type,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ValidatorTransactionType {
    #[serde(rename = "observed_jwk_update")]
    ObservedJwkUpdate,
}

impl Default for ValidatorTransactionType {
    fn default() -> ValidatorTransactionType {
        Self::ObservedJwkUpdate
    }
}

