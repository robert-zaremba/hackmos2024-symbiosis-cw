use cosmwasm_std::{Addr, Coin, Deps, Order, StdResult};

use crate::state::*;

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

pub type LeaderboardResp = Vec<(Addr, Vec<Coin>)>;

/// Returns all rewards from all users
pub fn leaderboard(deps: Deps) -> StdResult<LeaderboardResp> {
    let mut all: LeaderboardResp = vec![];
    let mut user = Addr::unchecked("");
    let mut user_coins: Vec<Coin> = Vec::new();
    for e in REWARDS.range(deps.storage, None, None, Order::Ascending) {
        let ((addr, denom), amount) = e.unwrap();
        if user != addr {
            if !user_coins.is_empty() {
                all.push((user, user_coins));
                user_coins = vec![];
            }
            user = addr;
        }
        user_coins.push(Coin { denom, amount });
    }
    if !user_coins.is_empty() {
        all.push((user, user_coins));
    }
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
