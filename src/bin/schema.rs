use cosmwasm_schema::write_api;

use hackmos_affiliate::contract::{Execute, InstantiateMsg, Query};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: Execute,
        query: Query,
    }
}
