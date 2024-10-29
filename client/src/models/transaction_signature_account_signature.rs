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
pub struct TransactionSignatureAccountSignature {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "public_key")]
    pub public_key: Box<models::PublicKey>,
    #[serde(rename = "signature")]
    pub signature: Box<models::Signature>,
    #[serde(rename = "public_keys")]
    pub public_keys: Vec<models::PublicKey>,
    #[serde(rename = "signatures")]
    pub signatures: Vec<models::IndexedSignature>,
    /// The number of signatures required for a successful transaction
    #[serde(rename = "threshold")]
    pub threshold: i32,
    /// All bytes (Vec<u8>) data is represented as hex-encoded string prefixed with `0x` and fulfilled with two hex digits per byte.  Unlike the `Address` type, HexEncodedBytes will not trim any zeros. 
    #[serde(rename = "bitmap")]
    pub bitmap: String,
    #[serde(rename = "signatures_required")]
    pub signatures_required: i32,
}

impl TransactionSignatureAccountSignature {
    pub fn new(r#type: Type, public_key: models::PublicKey, signature: models::Signature, public_keys: Vec<models::PublicKey>, signatures: Vec<models::IndexedSignature>, threshold: i32, bitmap: String, signatures_required: i32) -> TransactionSignatureAccountSignature {
        TransactionSignatureAccountSignature {
            r#type,
            public_key: Box::new(public_key),
            signature: Box::new(signature),
            public_keys,
            signatures,
            threshold,
            bitmap,
            signatures_required,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "single_sender")]
    SingleSender,
}

impl Default for Type {
    fn default() -> Type {
        Self::SingleSender
    }
}

