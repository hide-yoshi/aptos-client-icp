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

/// MultisigPayload : A multisig transaction that allows an owner of a multisig account to execute a pre-approved transaction as the multisig account.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultisigPayload {
    /// A hex encoded 32 byte Aptos account address.  This is represented in a string as a 64 character hex string, sometimes shortened by stripping leading 0s, and adding a 0x.  For example, address 0x0000000000000000000000000000000000000000000000000000000000000001 is represented as 0x1. 
    #[serde(rename = "multisig_address")]
    pub multisig_address: String,
    #[serde(rename = "transaction_payload", skip_serializing_if = "Option::is_none")]
    pub transaction_payload: Option<Box<models::MultisigTransactionPayload>>,
}

impl MultisigPayload {
    /// A multisig transaction that allows an owner of a multisig account to execute a pre-approved transaction as the multisig account.
    pub fn new(multisig_address: String) -> MultisigPayload {
        MultisigPayload {
            multisig_address,
            transaction_payload: None,
        }
    }
}

