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

/// GasEstimation : Struct holding the outputs of the estimate gas API
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GasEstimation {
    /// The deprioritized estimate for the gas unit price
    #[serde(rename = "deprioritized_gas_estimate", skip_serializing_if = "Option::is_none")]
    pub deprioritized_gas_estimate: Option<i32>,
    /// The current estimate for the gas unit price
    #[serde(rename = "gas_estimate")]
    pub gas_estimate: i32,
    /// The prioritized estimate for the gas unit price
    #[serde(rename = "prioritized_gas_estimate", skip_serializing_if = "Option::is_none")]
    pub prioritized_gas_estimate: Option<i32>,
}

impl GasEstimation {
    /// Struct holding the outputs of the estimate gas API
    pub fn new(gas_estimate: i32) -> GasEstimation {
        GasEstimation {
            deprioritized_gas_estimate: None,
            gas_estimate,
            prioritized_gas_estimate: None,
        }
    }
}

