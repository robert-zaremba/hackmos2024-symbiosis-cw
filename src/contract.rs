#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute;
use crate::query;
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:hackmos-affiliate";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cw_serde]
pub struct InstantiateMsg {
    pub community_fund: Addr,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        next_aff_id: 1,
        community_fund: msg.community_fund,
        fee_p: 10,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cw_serde]
pub enum Execute {
    NewAffiliate { parent: Addr },
    DistributeRewards { user: Addr },
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: Execute,
) -> Result<Response, ContractError> {
    match msg {
        Execute::NewAffiliate { parent } => execute::new_affiliate(deps, info.sender, parent),
        Execute::DistributeRewards { user } => execute::distribute_rewards(deps, info, user),
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum Query {
    #[returns(query::RewardsResp)]
    Rewards { user: Addr },
    #[returns(query::AffiliatesResp)]
    Affiliates { user: Addr },
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: Query) -> StdResult<Binary> {
    match msg {
        Query::Rewards { user } => to_json_binary(&query::rewards(deps, user)?),
        Query::Affiliates { user } => to_json_binary(&query::affiliates(deps, user)?),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{
        message_info, mock_dependencies, mock_env, MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::{coins, from_json, Coin, Empty, OwnedDeps};
    use query::{AffiliatesResp, RewardsResp};

    fn addr_cf(api: MockApi) -> Addr {
        // Addr::unchecked("community_fund")
        api.addr_make("cf")
    }
    fn addr_alice(api: MockApi) -> Addr {
        api.addr_make("alice")
    }
    fn addr_bob(api: MockApi) -> Addr {
        api.addr_make("bob")
    }
    fn addr_parent1(api: MockApi) -> Addr {
        api.addr_make("parent1")
    }
    fn addr_parent2(api: MockApi) -> Addr {
        api.addr_make("parent2")
    }

    type DepsType = OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>;

    fn setup() -> DepsType {
        let mut deps = mock_dependencies();
        let community_fund = addr_cf(deps.api);
        let msg = InstantiateMsg { community_fund };
        let creator = deps.api.addr_make("creator");
        let info = message_info(&creator, &vec![]);

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        deps
    }

    #[test]
    fn proper_initialization() {
        let deps = setup();
        let res = query(
            deps.as_ref(),
            mock_env(),
            Query::Rewards {
                user: addr_alice(deps.api),
            },
        )
        .unwrap();
        let expected: RewardsResp = from_json(&res).unwrap();
        let empty: RewardsResp = Vec::new();
        assert_eq!(empty, expected);
        let resp = query::affiliates(deps.as_ref(), addr_alice(deps.api));
        assert_eq!(resp, Ok(vec![]));
    }

    #[test]
    fn affiliate() {
        let mut deps = setup();

        // let info = mock_info("anyone", &coins(2, "token"));

        let alice = addr_alice(deps.api);
        let parent1 = addr_parent1(deps.api);
        let parent2 = addr_parent2(deps.api);

        let res = execute::new_affiliate(deps.as_mut(), parent1.clone(), parent2.clone());
        assert!(matches!(res, Ok(_)));
        let res = execute::new_affiliate(deps.as_mut(), alice.clone(), parent1.clone());
        assert!(matches!(res, Ok(_)));
        let res = execute::new_affiliate(deps.as_mut(), alice.clone(), parent1.clone());
        assert!(matches!(res, Err(ContractError::AlreadyAffiliated {})));

        let res = query::affiliates(deps.as_ref(), parent2.clone());
        assert_eq!(res, Ok(vec![]));

        let res = query::affiliates(deps.as_ref(), parent1.clone());
        assert_eq!(res, Ok(vec![parent2.clone()]));

        let res = query::affiliates(deps.as_ref(), alice.clone());
        let expected_alice = vec![parent1.clone(), parent2.clone()];
        assert_eq!(res, Ok(expected_alice.clone()));

        // e2e test including JS
        let res_bin = query(deps.as_ref(), mock_env(), Query::Affiliates { user: alice }).unwrap();
        let res: AffiliatesResp = from_json(&res_bin).unwrap();
        assert_eq!(res, expected_alice);
    }

    // #[test]
    // fn reset() {
    //     let mut deps = mock_dependencies();

    //     let msg = InstantiateMsg { count: 17 };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // beneficiary can release it
    //     let unauth_info = mock_info("anyone", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
    //     match res {
    //         Err(ContractError::Unauthorized {}) => {}
    //         _ => panic!("Must return unauthorized error"),
    //     }

    //     // only the original creator can reset the counter
    //     let auth_info = mock_info("creator", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

    //     // should now be 5
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: GetCountResponse = from_json(&res).unwrap();
    //     assert_eq!(5, value.count);
    // }
}
