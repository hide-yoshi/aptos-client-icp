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
pub struct AccountSignatureNoAccountSignature {
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl AccountSignatureNoAccountSignature {
    pub fn new(r#type: Type) -> AccountSignatureNoAccountSignature {
        AccountSignatureNoAccountSignature {
            r#type,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "no_account_signature")]
    NoAccountSignature,
}

impl Default for Type {
    fn default() -> Type {
        Self::NoAccountSignature
    }
}

