import "dotenv/config";
import { LCDClient, MnemonicKey } from "@terra-money/terra.js";
import { MsgExecuteContract } from "@terra-money/terra.js";

const contract =
  "terra1wn625s4jcmvk0szpl85rj5azkfc6suyvf75q6vrddscjdphtve8stalnth";

const sig_bytes =
  "448792aef0812810b2ce8d322830437768786ae7c453c4486b4231a687f009e4651230cc5981acdbc77aac3f50faa0f4758bdf67269c59bce82462d50f59db3a0000";

const resp_bytes =
  "010000473a97cf96a58964fa95d002062341408e908ae04e52fd2ec3fca5c3d8d2fcc07fb8746cbef127d1106042b78a561069de22469f88bd8565b88eafb628e9f44c000000004f0100000001010002010000004200000005307832383002ddb64fe46a91d46ee29420539fc25fd07c5fea3e0000000406fdde03ddb64fe46a91d46ee29420539fc25fd07c5fea3e00000004313ce56701000201000000b9000000000000028055b5ea5057a9ceffb928b2a5f1bab720eb03748cd64d65e60e73854dc4ab2e1f00061d71fa699f8002000000600000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000d5772617070656420457468657200000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000012";

const hash = "d50d15c01844d9b986d958a30a6cbef39f43e979250264db6cff17481d5c9097";

/* Set up terra client & wallet */
const terra = new LCDClient({
  URL: "http://localhost:1318",
  chainID: "localterra",
});

// const wallet = terra.wallet(
//   new MnemonicKey({
//     mnemonic:
//       "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius",
//   })
// );

// await wallet.sequence();

// console.log("Doing verify using execute!");

// await wallet
//   .createAndSignTx({
//     msgs: [
//       new MsgExecuteContract(
//         wallet.key.accAddress,
//         contract,
//         {
//           verify_signatures_from_hash: {
//             sig_bytes: Buffer.from(sig_bytes, "hex").toString("base64"),
//             hash: Buffer.from(hash, "hex").toString("base64"),
//           },
//         },
//         { uluna: 1000 }
//       ),
//     ],
//     memo: "",
//   })
//   .then((tx) => terra.tx.broadcast(tx))
//   .then((rs) => console.log("Success: ", rs))
//   .catch((error) => {
//     if (error.response) {
//       // Request made and server responded
//       console.error(
//         "Error response: ",
//         error.response.data,
//         error.response.status,
//         error.response.headers
//       );
//     } else if (error.request) {
//       // The request was made but no response was received
//       console.error("No response received: ", error.request);
//     } else {
//       // Something happened in setting up the request that triggered an Error
//       console.error("Error", error.message);
//     }

//     throw new Error(`Failed to verify signatures: ${error}`);
//   });

console.log("Doing verify from hash");
try {
  const resp = await terra.wasm.contractQuery(contract, {
    verify_signatures_from_hash: {
      sig_bytes: Buffer.from(sig_bytes, "hex").toString("base64"),
      hash: Buffer.from(hash, "hex").toString("base64"),
    },
  });
  console.log("Success: ", resp);
} catch (e) {
  console.error("Query failed: ", e);
}

console.log("Doing verify from response bytes");

try {
  const resp = await terra.wasm.contractQuery(contract, {
    verify_signatures: {
      sig_bytes: Buffer.from(sig_bytes, "hex").toString("base64"),
      resp_bytes: Buffer.from(resp_bytes, "hex").toString("base64"),
    },
  });
  console.log("Success: ", resp);
} catch (e) {
  console.error("Query failed: ", e);
}

console.log("Doing totalSupply query on mainnet WETH");

const ts_sig_bytes =
  "f6f7718691587463a4fd5a0a83b8e79f90d3c0a865e906d2b1a57817fc23685400ab16efc0bfe6b25bd114f1d0f38c075d89d191c290407eb3b7392e5e87e13e0000";

const ts_resp_bytes =
  "010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000037010000002a010002010000002a0000000930783133363463306201c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000418160ddd01000201000000550000000001364c0b6c6c01c7fb4ad573a674025202155f3a60829e780d7312b9e131105ea068ac2900061d8af2b674c001000000200000000000000000000000000000000000000000000242ad82df886ac1ea6ca5";

try {
  const resp = await terra.wasm.contractQuery(contract, {
    weth_total_supply: {
      sig_bytes: Buffer.from(ts_sig_bytes, "hex").toString("base64"),
      resp_bytes: Buffer.from(ts_resp_bytes, "hex").toString("base64"),
    },
  });
  console.log("Success: ", resp);
} catch (e) {
  console.error("Total Supply call failed: ", e);
}
