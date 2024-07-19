#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, QuerierWrapper, QueryRequest, Response, StdResult, Storage, WasmQuery,
};

use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

use k256::{
    ecdsa::{
        recoverable::{Id as RecoverableId, Signature as RecoverableSignature},
        Signature, VerifyingKey,
    },
    elliptic_curve::sec1::{FromEncodedPoint, ToEncodedPoint},
    AffinePoint, EncodedPoint,
};
use sha3::{Digest, Keccak256};

use generic_array::GenericArray;
use std::convert::TryFrom;

use cw_wormhole::{
    msg::{GuardianSetInfoResponse, QueryMsg as WormholeQueryMsg},
    state::GuardianAddress,
};

use wormhole_query_sdk::{
    structs::{ChainSpecificQuery,ChainSpecificResponse, QueryResponse},
    MESSAGE_PREFIX,
};

use crate::{
    error::ExampleQueriesError,
    msg::{ExecuteMsg, ComputeHashResponse, InstantiateMsg, QueryMsg, WethTotalSupplyResponse},
    state::{CONFIG_INFO, ConfigInfo, GUARDIAN_INDEX_OFFSET, SIGNATURE_LEN, SIG_DATA_LEN, SIG_RECOVERY_ID_OFFSET},
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, anyhow::Error> {
    // Save general wormhole info
    let state = ConfigInfo {
        wormhole_contract: msg.wormhole_contract,
    };
    CONFIG_INFO.save(deps.storage, &state)?;
        
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, anyhow::Error> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::VerifySignatures { sig_bytes, resp_bytes } => to_json_binary(&handle_verify_signatures(deps.storage, deps.querier, env, sig_bytes, resp_bytes)?),    
        QueryMsg::VerifySignaturesFromHash { sig_bytes, hash } => to_json_binary(&handle_verify_signatures_from_hash(deps.storage, deps.querier, env, sig_bytes, hash)?),
        QueryMsg::ComputeHash { data } => to_json_binary(&compute_hash(data.as_slice())?),
        QueryMsg::WethTotalSupply { sig_bytes, resp_bytes } => to_json_binary(&handle_weth_total_supply(deps.storage, deps.querier, env, sig_bytes, resp_bytes)?),  
    }
}

fn handle_verify_signatures(
    storage: &dyn Storage,
    querier: QuerierWrapper,
    _env: Env,
    sig_bytes: Binary,
    resp_bytes: Binary,
) -> StdResult<Response> {
    let cfg = CONFIG_INFO.load(storage)?;
    let guardian_set: GuardianSetInfoResponse = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: cfg.wormhole_contract,
        msg: to_json_binary(&WormholeQueryMsg::GuardianSetInfo {})?,
    }))?;
    verify_signatures(sig_bytes.as_slice(), &guardian_set, resp_bytes.as_slice())
}

fn handle_verify_signatures_from_hash(
    storage: &dyn Storage,
    querier: QuerierWrapper,
    _env: Env,
    sig_bytes: Binary,
    hash: Binary,
) -> StdResult<Response> {
    let cfg = CONFIG_INFO.load(storage)?;
    let guardian_set: GuardianSetInfoResponse = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: cfg.wormhole_contract,
        msg: to_json_binary(&WormholeQueryMsg::GuardianSetInfo {})?,
    }))?;
    verify_signatures_from_hash(sig_bytes.as_slice(), &guardian_set, hash.as_slice())
}

pub fn verify_signatures(
    sig_bytes: &[u8],
    guardian_set: &GuardianSetInfoResponse,
    resp_bytes: &[u8],
) -> StdResult<Response> {
    let resp = compute_hash(resp_bytes).unwrap();
    verify_signatures_from_hash(sig_bytes, &guardian_set, resp.hash.as_slice())
}

// verify_signatures_from_hash is heavily based on parse_and_verify_vaa in cw_wormhole.
pub fn verify_signatures_from_hash(
    sig_bytes: &[u8],
    guardian_set: &GuardianSetInfoResponse,
    hash: &[u8],
) -> StdResult<Response> {
    // Compute the number of signatures and verify there are no extra bytes.
    let num_sigs = sig_bytes.len() / SIGNATURE_LEN;
    let expected_len = num_sigs * SIGNATURE_LEN;
    if expected_len != sig_bytes.len() {
        return ExampleQueriesError::InvalidSigLen.std_err();
    }

    // Make sure we have enough signatures to meet quorum.
    let quorum = (guardian_set.addresses.len() * 2) / 3 + 1;
    if num_sigs < quorum {
        return ExampleQueriesError::NoQuorum.std_err();
    }

    let mut last_index: i32 = -1;
    let mut pos = 0;

    for _ in 0..num_sigs {
        if pos + SIGNATURE_LEN > sig_bytes.len() {
            return ExampleQueriesError::InvalidSigLen2.std_err();
        }
        let index = sig_bytes[pos + GUARDIAN_INDEX_OFFSET] as i32;
        if index <= last_index {
            return ExampleQueriesError::WrongGuardianIndexOrder.std_err();
        }
        last_index = index;
        let idx = index as usize;

        let signature = Signature::try_from(
            &sig_bytes[pos..pos + SIG_DATA_LEN],
        )
        .or_else(|_| ExampleQueriesError::CannotDecodeSignature.std_err())?;

        let id = RecoverableId::new(sig_bytes[pos + SIG_RECOVERY_ID_OFFSET])
            .or_else(|_| ExampleQueriesError::CannotDecodeSignature.std_err())?;
        
        // verify_signature(&signature, index as usize, &id, &guardian_set, &hash).unwrap();

                // verify_signature(&signature, index as usize, &id, &guardian_set, &hash).unwrap();
        let recoverable_signature = RecoverableSignature::new(&signature, id)
            .or_else(|_| ExampleQueriesError::CannotDecodeSignature.std_err())?;

        let verify_key = recoverable_signature
            .recover_verifying_key_from_digest_bytes(GenericArray::from_slice(hash))
            .or_else(|_| ExampleQueriesError::CannotRecoverKey.std_err())?;

        if idx >= guardian_set.addresses.len() {
            return ExampleQueriesError::TooManySignatures.std_err();
        }
        if !keys_equal(&verify_key, &guardian_set.addresses[idx]) {
            return ExampleQueriesError::GuardianSignatureError.std_err();
        } 

        pos += SIGNATURE_LEN;
    }

    Ok(Response::new())
}

// keys_equal is copied directly from cw_wormhole core.
fn keys_equal(a: &VerifyingKey, b: &GuardianAddress) -> bool {
    let mut hasher = Keccak256::new();

    let affine_point_option = AffinePoint::from_encoded_point(&EncodedPoint::from(a));
    let affine_point = if affine_point_option.is_some().into() {
        affine_point_option.unwrap()
    } else {
        return false;
    };

    let decompressed_point = affine_point.to_encoded_point(false);

    hasher.update(&decompressed_point.as_bytes()[1..]);
    let a = &hasher.finalize()[12..];

    let b = &b.bytes;
    if a.len() != b.len() {
        return false;
    }
    for (ai, bi) in a.iter().zip(b.as_slice().iter()) {
        if ai != bi {
            return false;
        }
    }
    true
}

pub fn compute_hash(
    data: &[u8],
) -> StdResult<ComputeHashResponse> {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let hash = hasher.finalize().to_vec();

    // Prepend the prefix.
    let data = [
            MESSAGE_PREFIX,
            &hash]
        .concat();

    // Rehash the hash
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let hash = hasher.finalize().to_vec();

    let res = ComputeHashResponse{hash};
    Ok(res)
}

pub fn parse_and_verify_query_response(
   resp_bytes: &[u8],
) ->  StdResult<QueryResponse> {
    let response = QueryResponse::deserialize(&resp_bytes)
    .or_else(|_| ExampleQueriesError::FailedToParseResponse.std_err())?;

    Ok(response)
}

fn handle_weth_total_supply(
    storage: &dyn Storage,
    querier: QuerierWrapper,
    _env: Env,
    sig_bytes: Binary,
    resp_bytes: Binary,
) -> StdResult<WethTotalSupplyResponse> {
    // Get the current guardian set.
    let cfg = CONFIG_INFO.load(storage)?;
    let guardian_set: GuardianSetInfoResponse = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: cfg.wormhole_contract,
        msg: to_json_binary(&WormholeQueryMsg::GuardianSetInfo {})?,
    }))?;

    weth_total_supply(sig_bytes.as_slice(), &guardian_set, resp_bytes.as_slice())
}

pub fn weth_total_supply(
    sig_bytes: &[u8],
    guardian_set: &GuardianSetInfoResponse,
    resp_bytes: &[u8],
) -> StdResult<WethTotalSupplyResponse> {
    // Verify the signatures against the current guardian set.
    verify_signatures(sig_bytes, &guardian_set, resp_bytes).unwrap();

    // Parse the response.
    let resp = parse_and_verify_query_response(&resp_bytes).unwrap();

    if resp.request.requests.len() != 1 {
        return ExampleQueriesError::UnexpectedNumberOfRequests.std_err();
    }

    // TODO: Could allow the other eth query types.

    // Verify the request is for a totalSupply query. 
    let per_chain_req = &resp.request.requests[0];
    match &per_chain_req.query {
        ChainSpecificQuery::EthCallQueryRequest(eth_req) => {
            if eth_req.call_data.len() != 1 {
                return ExampleQueriesError::UnexpectedNumberOfCallData.std_err();
            }
            let call_data = &eth_req.call_data[0];
            if call_data.data.len() != 4 {
                 return ExampleQueriesError::UnexpectedCallDataLength.std_err();
            }   
            // totalSupply() is 0x18160ddd                       
            if call_data.data[0] != 0x18 || call_data.data[1] != 0x16 || call_data.data[2] != 0x0d || call_data.data[3] != 0xdd {
                return ExampleQueriesError::UnexpectedCallType.std_err();
            }            
        }
        ChainSpecificQuery::EthCallByTimestampQueryRequest(_) => {
            return ExampleQueriesError::UnsupportedRequestType.std_err();
        }
        ChainSpecificQuery::EthCallWithFinalityQueryRequest(_) => {
            return ExampleQueriesError::UnsupportedRequestType.std_err();
        }
        ChainSpecificQuery::SolanaAccountQueryRequest(_) => {
            return ExampleQueriesError::UnsupportedRequestType.std_err();
        }
    }

    if resp.responses.len() != 1 {
        return ExampleQueriesError::UnexpectedNumberOfResponses.std_err();
    }

    let per_chain_resp = &resp.responses[0];
    match &per_chain_resp.response {
        ChainSpecificResponse::EthCallQueryResponse(eth_response) => {
            if eth_response.results.len() != 1 {
                return ExampleQueriesError::UnexpectedNumberOfResults.std_err();
            }
            if eth_response.results[0].len() != 32 {
                return ExampleQueriesError::UnexpectedResultLength.std_err();
            }
            let mut rdr = Cursor::new(&eth_response.results[0][16..32]);
            let total_supply = rdr.read_u128::<BigEndian>()
                .or_else(|_| ExampleQueriesError::InvalidTotalSupply.std_err())?;
            let result = WethTotalSupplyResponse{total_supply: total_supply};
            return Ok(result)
        }
        ChainSpecificResponse::EthCallByTimestampQueryResponse(_) => {
            return ExampleQueriesError::UnsupportedResponseType.std_err();
        }
        ChainSpecificResponse::EthCallWithFinalityQueryResponse(_) => {
            return ExampleQueriesError::UnsupportedResponseType.std_err();
        }
        ChainSpecificResponse::SolanaAccountQueryResponse(_) => {
            return ExampleQueriesError::UnsupportedResponseType.std_err();
        }
    }
}