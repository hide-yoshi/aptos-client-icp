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
pub struct WriteSetScriptWriteSet {
    /// A hex encoded 32 byte Aptos account address.  This is represented in a string as a 64 character hex string, sometimes shortened by stripping leading 0s, and adding a 0x.  For example, address 0x0000000000000000000000000000000000000000000000000000000000000001 is represented as 0x1. 
    #[serde(rename = "execute_as")]
    pub execute_as: String,
    #[serde(rename = "script")]
    pub script: Box<models::ScriptPayload>,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl WriteSetScriptWriteSet {
    pub fn new(execute_as: String, script: models::ScriptPayload, r#type: Type) -> WriteSetScriptWriteSet {
        WriteSetScriptWriteSet {
            execute_as,
            script: Box::new(script),
            r#type,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "script_write_set")]
    ScriptWriteSet,
}

impl Default for Type {
    fn default() -> Type {
        Self::ScriptWriteSet
    }
}

