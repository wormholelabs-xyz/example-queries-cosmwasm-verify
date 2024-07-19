use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {
    pub wormhole_contract: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    VerifySignatures {
        // sig_bytes should be the byte array as returned in the query response. That is, an array of 66 bytes keys where the last byte is the guardian index.
        sig_bytes: Binary,

        // resp_bytes should be the response bytes returned in the query response.
        resp_bytes: Binary,
    },   
    VerifySignaturesFromHash {
        // sig_bytes should be the byte array as returned in the query response. That is, an array of 66 bytes keys where the last byte is the guardian index.
        sig_bytes: Binary,

        // hash should be the hash of the response.
        hash: Binary,
    },
    ComputeHash {
        // sig_bytes should be the byte array as returned in the query response. That is, an array of 66 bytes keys where the last byte is the guardian index.
        data: Binary,
    },
    WethTotalSupply {
        // sig_bytes should be the byte array as returned in the query response. That is, an array of 66 bytes keys where the last byte is the guardian index.
        sig_bytes: Binary,

        // resp_bytes should be the response bytes returned in the query response for a query of the `totalSupply` on the WETH contract.
        resp_bytes: Binary,
    },    
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ComputeHashResponse {
    pub hash: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct WethTotalSupplyResponse {
    pub total_supply: u128,
}
