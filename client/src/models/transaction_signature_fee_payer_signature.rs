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
pub struct TransactionSignatureFeePayerSignature {
    #[serde(rename = "sender")]
    pub sender: Box<models::AccountSignature>,
    /// The other involved parties' addresses
    #[serde(rename = "secondary_signer_addresses")]
    pub secondary_signer_addresses: Vec<String>,
    /// The associated signatures, in the same order as the secondary addresses
    #[serde(rename = "secondary_signers")]
    pub secondary_signers: Vec<models::AccountSignature>,
    #[serde(rename = "fee_payer_address")]
    pub fee_payer_address: Box<String>,
    #[serde(rename = "fee_payer_signer")]
    pub fee_payer_signer: Box<models::FeePayerSignatureFeePayerSigner>,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl TransactionSignatureFeePayerSignature {
    pub fn new(sender: models::AccountSignature, secondary_signer_addresses: Vec<String>, secondary_signers: Vec<models::AccountSignature>, fee_payer_address: String, fee_payer_signer: models::FeePayerSignatureFeePayerSigner, r#type: Type) -> TransactionSignatureFeePayerSignature {
        TransactionSignatureFeePayerSignature {
            sender: Box::new(sender),
            secondary_signer_addresses,
            secondary_signers,
            fee_payer_address: Box::new(fee_payer_address),
            fee_payer_signer: Box::new(fee_payer_signer),
            r#type,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "fee_payer_signature")]
    FeePayerSignature,
}

impl Default for Type {
    fn default() -> Type {
        Self::FeePayerSignature
    }
}

