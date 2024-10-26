use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Deps, Order, StdResult};

use crate::state::{AFF_PARENTS, MAX_PARENTS, REWARDS};

// use crate::state::*;

// We define a custom struct for each query response
#[cw_serde]
pub struct RewardsResp2 {
    pub xx: u64,
}

pub type RewardsResp = Vec<Coin>;

pub fn rewards(deps: Deps, user: Addr) -> StdResult<RewardsResp> {
    let all: RewardsResp = REWARDS
        .prefix(user)
        .range(deps.storage, None, None, Order::Ascending)
        .map(|x| {
            // TODO: don't unwrap
            let (denom, amount) = x.unwrap();
            return Coin { denom, amount };
        })
        .collect();
    Ok(all)
}

pub type AffiliatesResp = Vec<Addr>;
pub fn affiliates(deps: Deps, user: Addr) -> StdResult<AffiliatesResp> {
    let mut parent = user;
    let mut res = vec![];
    for _i in 1..=MAX_PARENTS {
        let next_parent = AFF_PARENTS.load(deps.storage, parent.clone());
        if next_parent.is_err() {
            break;
        }
        parent = next_parent.unwrap();
        res.push(parent.clone());
    }
    Ok(res)
}
