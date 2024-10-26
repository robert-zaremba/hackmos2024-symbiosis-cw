use cosmwasm_std::{Addr, StdResult, Uint128};
use cosmwasm_std::{DepsMut, MessageInfo, Response};

use crate::error::ContractError;
use crate::state::*;

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
    info: MessageInfo,
    user: Addr,
) -> Result<Response, ContractError> {
    let s = STATE.load(deps.storage)?;
    let fee_factor = (s.fee_p as u128, 2u128);
    let mut funds = info.funds.clone();
    let mut parent = user;
    for i in 1..=MAX_PARENTS {
        let mut use_all = i == MAX_PARENTS;
        let next_parent = AFF_PARENTS.load(deps.storage, parent.clone());
        if next_parent.is_err() {
            use_all = true;
            parent = s.community_fund.clone();
        } else {
            parent = next_parent.unwrap();
        }
        for c in &mut funds {
            let cut = c.amount.mul_floor(fee_factor);
            c.amount -= cut;
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
        if use_all {
            break;
        }
    }

    Ok(Response::new().add_attribute("action", "distribute_rewards"))
}
