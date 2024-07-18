use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;

type HumanAddr = String;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ConfigInfo {
    /// Address of the core bridge contract
    pub wormhole_contract: HumanAddr,
}

pub const CONFIG_INFO: Item<ConfigInfo> = Item::new("config_info");

// Total length of each signature entry, including the guardian index at the end.
pub const SIGNATURE_LEN: usize = 66;

// The length of the signature data, excluding the recovery id.
pub const SIG_DATA_LEN: usize = 64;

// The offset to the recovery id.
pub const SIG_RECOVERY_ID_OFFSET: usize = SIG_DATA_LEN;

// The offset to the guardian index.
pub const GUARDIAN_INDEX_OFFSET: usize = SIG_RECOVERY_ID_OFFSET + 1;
