use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Deps, StdResult};

// use crate::state::*;

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}

pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
    // let state = STATE.load(deps.storage)?;
    Ok(GetCountResponse { count: 1 })
}
