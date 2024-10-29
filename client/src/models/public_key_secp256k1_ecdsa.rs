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
pub struct PublicKeySecp256k1Ecdsa {
    /// All bytes (Vec<u8>) data is represented as hex-encoded string prefixed with `0x` and fulfilled with two hex digits per byte.  Unlike the `Address` type, HexEncodedBytes will not trim any zeros. 
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl PublicKeySecp256k1Ecdsa {
    pub fn new(value: String, r#type: Type) -> PublicKeySecp256k1Ecdsa {
        PublicKeySecp256k1Ecdsa {
            value,
            r#type,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "secp256k1_ecdsa")]
    Secp256k1Ecdsa,
}

impl Default for Type {
    fn default() -> Type {
        Self::Secp256k1Ecdsa
    }
}

