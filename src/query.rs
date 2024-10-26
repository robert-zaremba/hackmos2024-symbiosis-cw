use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Deps, StdResult};

use crate::state::REWARDS;

// use crate::state::*;

// We define a custom struct for each query response
#[cw_serde]
pub struct RewardsResp2 {
    pub xx: u64,
}

pub type RewardsResp = Vec<Coin>;

pub fn rewards(deps: Deps, user: Addr) -> StdResult<RewardsResp> {
    // let state = STATE.load(deps.storage)?;
    Ok(vec![])
}
