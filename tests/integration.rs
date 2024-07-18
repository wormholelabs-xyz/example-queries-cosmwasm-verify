use cw_wormhole::{
    msg::GuardianSetInfoResponse,
    state::GuardianAddress,
};

use wormhole_query_sdk::structs::ChainSpecificResponse;

use example_queries_cosmwasm_verify::contract::{compute_hash, parse_and_verify_query_response, verify_signatures, verify_signatures_from_hash, weth_total_supply};

#[test]
fn compute_hash_works() {
    let data = "010000473a97cf96a58964fa95d002062341408e908ae04e52fd2ec3fca5c3d8d2fcc07fb8746cbef127d1106042b78a561069de22469f88bd8565b88eafb628e9f44c000000004f0100000001010002010000004200000005307832383002ddb64fe46a91d46ee29420539fc25fd07c5fea3e0000000406fdde03ddb64fe46a91d46ee29420539fc25fd07c5fea3e00000004313ce56701000201000000b9000000000000028055b5ea5057a9ceffb928b2a5f1bab720eb03748cd64d65e60e73854dc4ab2e1f00061d71fa699f8002000000600000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000d5772617070656420457468657200000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000012";
    let data = hex::decode(data).expect("data decode failed");
    let data = data.as_slice();
    
    let hash = "d50d15c01844d9b986d958a30a6cbef39f43e979250264db6cff17481d5c9097";
    let hash = hex::decode(hash).expect("hash decode failed");

    let res = compute_hash(&data).unwrap();
    assert_eq!(hash, res.hash);
}

#[test]
fn verify_signatures_from_hash_works_for_devnet_query() {
    let guardians = [GuardianAddress {
        bytes: hex::decode("beFA429d57cD18b7F8A4d91A2da9AB4AF05d0FBe")
            .expect("guardian_addr decode failed")
            .into(),
    }];

    let guardian_set = GuardianSetInfoResponse{
        guardian_set_index: 0,
        addresses: guardians.to_vec(),
    };

    // 64 bytes of signature data, one byte of recovery ID and the last byte is the guardian index.
    let sig_bytes = "448792aef0812810b2ce8d322830437768786ae7c453c4486b4231a687f009e4651230cc5981acdbc77aac3f50faa0f4758bdf67269c59bce82462d50f59db3a0000";
    let sig_bytes = hex::decode(sig_bytes).expect("sig_bytes decode failed");
    let sig_bytes = sig_bytes.as_slice();
    
    let hash = "d50d15c01844d9b986d958a30a6cbef39f43e979250264db6cff17481d5c9097";
    let hash = hex::decode(hash).expect("hash decode failed");

    verify_signatures_from_hash(&sig_bytes, &guardian_set, &hash).unwrap();
}

#[test]
fn verify_signatures_works_for_devnet_query() {
    let guardians = [GuardianAddress {
        bytes: hex::decode("beFA429d57cD18b7F8A4d91A2da9AB4AF05d0FBe")
            .expect("guardian_addr decode failed")
            .into(),
    }];

    let guardian_set = GuardianSetInfoResponse{
        guardian_set_index: 0,
        addresses: guardians.to_vec(),
    };

    // 64 bytes of signature data, one byte of recovery ID and the last byte is the guardian index.
    let sig_bytes = "448792aef0812810b2ce8d322830437768786ae7c453c4486b4231a687f009e4651230cc5981acdbc77aac3f50faa0f4758bdf67269c59bce82462d50f59db3a0000";
    let sig_bytes = hex::decode(sig_bytes).expect("sig_bytes decode failed");
    let sig_bytes = sig_bytes.as_slice();
    
    let resp_bytes = "010000473a97cf96a58964fa95d002062341408e908ae04e52fd2ec3fca5c3d8d2fcc07fb8746cbef127d1106042b78a561069de22469f88bd8565b88eafb628e9f44c000000004f0100000001010002010000004200000005307832383002ddb64fe46a91d46ee29420539fc25fd07c5fea3e0000000406fdde03ddb64fe46a91d46ee29420539fc25fd07c5fea3e00000004313ce56701000201000000b9000000000000028055b5ea5057a9ceffb928b2a5f1bab720eb03748cd64d65e60e73854dc4ab2e1f00061d71fa699f8002000000600000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000d5772617070656420457468657200000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000012";
    let resp_bytes = hex::decode(resp_bytes).expect("resp_bytes decode failed");
    let resp_bytes = resp_bytes.as_slice();

    verify_signatures(&sig_bytes, &guardian_set, &resp_bytes).unwrap();
}

#[test]
fn verify_signatures_works_for_mainnet_query() {
    // The data for this test comes from the `example-queries-solana-verify` repo, `example-queries-solana-verify.ts`.
    let mainnet_guardians = [
        GuardianAddress {bytes: hex::decode("5893b5a76c3f739645648885bdccc06cd70a3cd3").expect("guardian_addr decode failed").into()},
        GuardianAddress {bytes: hex::decode("ff6cb952589bde862c25ef4392132fb9d4a42157").expect("guardian_addr decode failed").into()},
        GuardianAddress {bytes: hex::decode("114de8460193bdf3a2fcf81f86a09765f4762fd1").expect("guardian_addr decode failed").into()},
        GuardianAddress {bytes: hex::decode("107a0086b32d7a0977926a205131d8731d39cbeb").expect("guardian_addr decode failed").into()},
        GuardianAddress {bytes: hex::decode("8c82b2fd82faed2711d59af0f2499d16e726f6b2").expect("guardian_addr decode failed").into()},
        GuardianAddress {bytes: hex::decode("11b39756c042441be6d8650b69b54ebe715e2343").expect("guardian_addr decode failed").into()},
        GuardianAddress {bytes: hex::decode("54ce5b4d348fb74b958e8966e2ec3dbd4958a7cd").expect("guardian_addr decode failed").into()},
        GuardianAddress {bytes: hex::decode("15e7caf07c4e3dc8e7c469f92c8cd88fb8005a20").expect("guardian_addr decode failed").into()},
        GuardianAddress {bytes: hex::decode("74a3bf913953d695260d88bc1aa25a4eee363ef0").expect("guardian_addr decode failed").into()},
        GuardianAddress {bytes: hex::decode("000ac0076727b35fbea2dac28fee5ccb0fea768e").expect("guardian_addr decode failed").into()},
        GuardianAddress {bytes: hex::decode("af45ced136b9d9e24903464ae889f5c8a723fc14").expect("guardian_addr decode failed").into()},
        GuardianAddress {bytes: hex::decode("f93124b7c738843cbb89e864c862c38cddcccf95").expect("guardian_addr decode failed").into()},
        GuardianAddress {bytes: hex::decode("d2cc37a4dc036a8d232b48f62cdd4731412f4890").expect("guardian_addr decode failed").into()},
        GuardianAddress {bytes: hex::decode("da798f6896a3331f64b48c12d1d57fd9cbe70811").expect("guardian_addr decode failed").into()},
        GuardianAddress {bytes: hex::decode("71aa1be1d36cafe3867910f99c09e347899c19c3").expect("guardian_addr decode failed").into()},
        GuardianAddress {bytes: hex::decode("8192b6e7387ccd768277c17dab1b7a5027c0b3cf").expect("guardian_addr decode failed").into()},
        GuardianAddress {bytes: hex::decode("178e21ad2e77ae06711549cfbb1f9c7a9d8096e8").expect("guardian_addr decode failed").into()},
        GuardianAddress {bytes: hex::decode("5e1487f35515d02a92753504a8d75471b9f49edb").expect("guardian_addr decode failed").into()},
        GuardianAddress {bytes: hex::decode("6fbebc898f403e4773e95feb15e80c9a99c8348d").expect("guardian_addr decode failed").into()},
    ];

    let guardian_set = GuardianSetInfoResponse{
        guardian_set_index: 0,
        addresses: mainnet_guardians.to_vec(),
    };

    // The guardian index is the last byte of each line.
    let sig_bytes = "\
    f122af3db0ae62af57bc16f0b3e79c86cbfc860a5994ca65928c06a739a2f4ca0496c7c1de38350e7b7cdc573fa0b7af981f3ac3d60298d67c76ca99d3bcf1040002\
    7b9af5d9a3438b5d44e04b7ae8c64894b8ea6a94701bf048bd106a3c79a6d2896843dae20b8db3fea62520565ddaf95a24d77783dfd990f7dc60a1a5c39d16840103\
    1a86399f16aee73e4aac7d9b06359805a818dd753cd3be77d7934a086f32b6d15d9166fa2d30af365c92bd6a8500c94a377d30a4b64741326f220ea920f4ecc20104\
    d4e9a063e8c015bf33081f2e37b3379870d5de6798d40694a69e92dcf66264540c84b26737617b93742b74d55068295c68ab7630efa8dc4f6d40b9c30ff17fb40006\
    998f80bd8c4f30ad30850782e9aaa24212470e233d48a126f3b174e241d8668872d0c37d306aecd15a6e740306bb625e31692ab1c58e89fe6030fa00b1e34c4d0107\
    59a772f2626f7376ff8a5279cea20290b625febd9b0dc8c312fcf59a3427445b4a97acbfe9394eacd709a6c49763bcb9d6bf7464f32020338a0f2edc824864f00109\
    4160ea981f0c5c1e9677aea518e5e999216dc6320b92037aea92266975468e9b2be7e73594f8e5b58290f57d7d0875654da779f38e1b167d06f71fead234d4a3010a\
    634f00406ff3d8ef65c5cb12bdb7cccdbc8da65025775e3a1f230ec167033de719dcdddb103c98be132478d559c4d8ee0b73f74bd89b06d525d4f6f09e8048c6000c\
    e7580e30907d0077951b62febd93daf3e9ae1887fe7b23c7a06354bb9aefb73c5613cbffb64e9887de71a90ab534533613f4b728a902a0be908e33b2bc070909010d\
    23fe620935057eab2e45cfeea8965985c0f3c96122ed1d12df3f39d1484eaeb940ad4dc225825fb68231384a094d420930f5060061b6dec71df4f1c752184a4a010e\
    ed986adf2099a6dc08bed9b6260d72bccf3e2226d774464b4761e7f885ff765d0d5291f1429b14862a52b6991a95fa6b842b66c2c3459970db2f314a1acd27710110\
    51b5c3b2f16104357ebce559f145ec0f6c1fcbec205dfcaefc1f131191e17fca0eb4eb76b6ff6550d1091644e00314ecd8aa94701e2ef8f00e5b62482710ef3c0011\
    a3d0cba06bf40ed5a3cc858dde5d3ab5ad016b242c273532c5b1419efe5863ae35d315a7087d6f0592c4dc3a7fccb4b6f1893af558a282728f5d9f468921ffd70012\
    ";
    let sig_bytes = hex::decode(sig_bytes).expect("sig_bytes decode failed");
    let sig_bytes = sig_bytes.as_slice();

    let resp_bytes = "01000051ced87ef0a0bb371964f793bb665a01435d57c9dc79b9fb6f31323f99f557ee0fa583718753cb3b35fe7c2e9bab2afde3f8cfdbeee0432804cb3c9146027a9401000000370100000001010002010000002a0000000930783132346330643601c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000406fdde030100020100000095000000000124c0d60f319af73bad19735c2f795e3bf22c0cb3d6be77b5fbd3bc1cf197efdbfb506c000610e4cf31cfc001000000600000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000d5772617070656420457468657200000000000000000000000000000000000000";
    let resp_bytes = hex::decode(resp_bytes).expect("resp_bytes decode failed");
    let resp_bytes = resp_bytes.as_slice();

    verify_signatures(&sig_bytes, &guardian_set, &resp_bytes).unwrap();
}

#[test]
fn parse_and_verify_query_response_works() {
    let resp_bytes = "01000051ced87ef0a0bb371964f793bb665a01435d57c9dc79b9fb6f31323f99f557ee0fa583718753cb3b35fe7c2e9bab2afde3f8cfdbeee0432804cb3c9146027a9401000000370100000001010002010000002a0000000930783132346330643601c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000406fdde030100020100000095000000000124c0d60f319af73bad19735c2f795e3bf22c0cb3d6be77b5fbd3bc1cf197efdbfb506c000610e4cf31cfc001000000600000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000d5772617070656420457468657200000000000000000000000000000000000000";
    let resp_bytes = hex::decode(resp_bytes).expect("resp_bytes decode failed");
    let resp_bytes = resp_bytes.as_slice();

    let resp: wormhole_query_sdk::structs::QueryResponse = parse_and_verify_query_response(&resp_bytes).unwrap();

    assert_eq!(1, resp.version);
    assert_eq!(0, resp.request_chain_id); // Zero means off chain request.

    // For off chain requests, the request ID is the signature of the request.
    let request_id = "51ced87ef0a0bb371964f793bb665a01435d57c9dc79b9fb6f31323f99f557ee0fa583718753cb3b35fe7c2e9bab2afde3f8cfdbeee0432804cb3c9146027a9401";
    let request_id = hex::decode(request_id).expect("request_id decode failed");
    assert_eq!(request_id, resp.request_id);

    assert_eq!(1, resp.responses.len());

    let pcqr = &resp.responses[0];
    assert_eq!(2, pcqr.chain_id);
    match &pcqr.response {
        ChainSpecificResponse::EthCallQueryResponse(eth_response) => {
            assert_eq!(0x0124c0d6, eth_response.block_number);
            let block_hash = "0f319af73bad19735c2f795e3bf22c0cb3d6be77b5fbd3bc1cf197efdbfb506c";
            let block_hash = hex::decode(block_hash).expect("block_hash decode failed");
            let block_hash = block_hash.as_slice();            
            assert_eq!(block_hash, eth_response.block_hash);
            assert_eq!(0x000610e4cf31cfc0, eth_response.block_time);
            assert_eq!(1, eth_response.results.len());
            let result = "0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000d5772617070656420457468657200000000000000000000000000000000000000";
            let result = hex::decode(result).expect("result decode failed");
            let result = result.as_slice();
            assert_eq!(result, eth_response.results[0]);
        }
        ChainSpecificResponse::EthCallByTimestampQueryResponse(_) => {
            panic!("EthCallByTimestampQueryResponse unexpected")
        }
        ChainSpecificResponse::EthCallWithFinalityQueryResponse(_) => {
            panic!("EthCallWithFinalityQueryResponse unexpected")
        }
        ChainSpecificResponse::SolanaAccountQueryResponse(_) => {
            panic!("SolanaAccountQueryResponse unexpected")
        }
    }
}

#[test]
fn weth_total_supply_works() {
    let guardians = [GuardianAddress {
        bytes: hex::decode("beFA429d57cD18b7F8A4d91A2da9AB4AF05d0FBe")
            .expect("guardian_addr decode failed")
            .into(),
    }];

    let guardian_set = GuardianSetInfoResponse{
        guardian_set_index: 0,
        addresses: guardians.to_vec(),
    };

    // 64 bytes of signature data, one byte of recovery ID and the last byte is the guardian index.
    let sig_bytes = "11993001dd83574a5c5f78ffb869c3bf3636ae542d1cb7de7442eeed190c64465f19c3ca9df2467e07b147dadbbe4ef9520d3f61f1a8891d58e6537971ca69110000";
    let sig_bytes = hex::decode(sig_bytes).expect("sig_bytes decode failed");
    let sig_bytes = sig_bytes.as_slice();
    
    let resp_bytes = "01000031fd6b668d5c0a237e8294eb3de0025c9082ec691566bdfc8a8978f0931724367be17754357d3991f2e1f44b924ba0a6eee5ebca63618e64605b1c4e41edc32a01000000330100000001010002010000002600000005307833656401ddb64fe46a91d46ee29420539fc25fd07c5fea3e0000000418160ddd010002010000005500000000000003ed5f4a04cc1ab02eef18a25213c6a3b89cc6422b63c66ea1f85ac8bcd61df8d8b700061d76666c8b8001000000200000000000000000000000000000000000000000000000000000000000000000";
    let resp_bytes = hex::decode(resp_bytes).expect("resp_bytes decode failed");
    let resp_bytes = resp_bytes.as_slice();

    weth_total_supply(&sig_bytes, &guardian_set, &resp_bytes).unwrap();
}