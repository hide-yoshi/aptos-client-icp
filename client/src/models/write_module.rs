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

/// WriteModule : Write a new module or update an existing one
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WriteModule {
    /// A hex encoded 32 byte Aptos account address.  This is represented in a string as a 64 character hex string, sometimes shortened by stripping leading 0s, and adding a 0x.  For example, address 0x0000000000000000000000000000000000000000000000000000000000000001 is represented as 0x1. 
    #[serde(rename = "address")]
    pub address: String,
    /// State key hash
    #[serde(rename = "state_key_hash")]
    pub state_key_hash: String,
    #[serde(rename = "data")]
    pub data: Box<models::MoveModuleBytecode>,
}

impl WriteModule {
    /// Write a new module or update an existing one
    pub fn new(address: String, state_key_hash: String, data: models::MoveModuleBytecode) -> WriteModule {
        WriteModule {
            address,
            state_key_hash,
            data: Box::new(data),
        }
    }
}

