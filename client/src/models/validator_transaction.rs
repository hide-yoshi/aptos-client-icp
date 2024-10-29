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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "validator_transaction_type")]
pub enum ValidatorTransaction {
    #[serde(rename="observed_jwk_update")]
    ObservedJwkUpdate(Box<models::ValidatorTransactionJwkUpdateTransaction>),
    #[serde(rename="dkg_result")]
    DkgResult(Box<models::ValidatorTransactionDkgResultTransaction>),
}

impl Default for ValidatorTransaction {
    fn default() -> Self {
        Self::ObservedJwkUpdate(Default::default())
    }
}


