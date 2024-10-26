use cosmwasm_std::{Addr, Attribute, Coin, StdResult, Uint128};
use cosmwasm_std::{DepsMut, Response};

use crate::error::ContractError;
use crate::{query, state::*};

pub fn new_affiliate(deps: DepsMut, user: Addr, parent: Addr) -> Result<Response, ContractError> {
    if AFF_PARENTS.has(deps.storage, user.clone()) {
        return Err(ContractError::AlreadyAffiliated {});
    }

    AFF_PARENTS.save(deps.storage, user, &parent)?;
    let state = STATE.update(deps.storage, |mut s: State| -> Result<_, ContractError> {
        s.next_aff_id += 1;
        Ok(s)
    })?;

    Ok(Response::new()
        .add_attribute("action", "increment")
        .add_attribute("affiliate_id", state.next_aff_id.to_string()))
}

pub fn distribute_rewards(
    deps: DepsMut,
    user: Addr,
    funds: Vec<Coin>,
) -> Result<Response, ContractError> {
    let s = STATE.load(deps.storage)?;
    let fee_factor = (s.fee_p as u128, 100u128);
    let mut funds = funds.clone();
    let mut parents = query::affiliates(deps.as_ref(), user)?;
    if parents.is_empty() {
        parents.push(s.community_fund.clone());
    }
    let last_idx = parents.len() - 1;
    for (i, parent) in parents.iter().enumerate() {
        for c in &mut funds {
            let cut = if i == last_idx {
                c.amount
            } else {
                let cut = c.amount.mul_floor(fee_factor);
                c.amount -= cut;
                cut
            };
            REWARDS.update(
                deps.storage,
                (parent.clone(), c.denom.clone()),
                |r: Option<Uint128>| -> StdResult<_> {
                    if let Some(r) = r {
                        return Ok(r + cut);
                    }
                    return Ok(cut);
                },
            )?;
        }
    }

    let mut res = Response::new();
    res.attributes = parents
        .iter()
        .map(|p| Attribute::new("parent", p.to_string()))
        .collect();
    res.attributes
        .push(Attribute::new("action", "distribute_rewards"));
    Ok(res)
}
