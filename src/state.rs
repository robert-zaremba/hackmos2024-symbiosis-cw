use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, JsonSchema)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq, Eq, Clone))]
//#[cfg_attr(any(test, not(target_arch = "wasm32")), derive(Deserialize))]
pub struct State {
    pub next_aff_id: u64,
    pub community_fund: Addr,
    /// fee as percent value
    pub fee_p: u32,
}

pub const MAX_PARENTS: usize = 5;
pub const STATE: Item<State> = Item::new("0");
pub const REWARDS: Map<(Addr, String), Uint128> = Map::new("1");
/// maps affiliator address to his "parent"
pub const AFF_PARENTS: Map<Addr, Addr> = Map::new("2");
/// maps affiliation ID to affiliator address
pub const AFFILIATES: Map<u64, Addr> = Map::new("3");
