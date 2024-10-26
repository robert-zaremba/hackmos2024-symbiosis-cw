use cosmwasm_std::Addr;
use cosmwasm_std::{DepsMut, MessageInfo, Response};

use crate::error::ContractError;
use crate::state::*;

pub fn new_affiliate(deps: DepsMut, user: Addr, parent: Addr) -> Result<Response, ContractError> {
    // STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
    //     state.count += 1;
    //     Ok(state)
    // })?;

    Ok(Response::new().add_attribute("action", "increment"))
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
