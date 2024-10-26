use cosmwasm_std::Addr;
use cosmwasm_std::{DepsMut, MessageInfo, Response};

use crate::error::ContractError;
use crate::state::*;

pub fn new_affiliate(deps: DepsMut, user: Addr, parent: Addr) -> Result<Response, ContractError> {
    if AFF_PARENTS.has(deps.storage, user) {
        return Err(ContractError::AlreadyAffiliated {});
    }

    let state = STATE.update(deps.storage, |mut s: State| -> Result<_, ContractError> {
        s.next_aff_id += 1;
        Ok(s)
    })?;

    AFF_PARENTS.save(deps.storage, user, &parent);

    Ok(Response::new()
        .add_attribute("action", "increment")
        .add_attribute("affiliate_id", state.next_aff_id))
}

pub fn distribute_rewards(
    deps: DepsMut,
    info: MessageInfo,
    dest: Addr,
) -> Result<Response, ContractError> {
    // STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
    //     if info.sender != state.owner {
    //         return Err(ContractError::Unauthorized {});
    //     }
    //     state.count = count;
    //     Ok(state)
    // })?;
    Ok(Response::new().add_attribute("action", "reset"))
}
