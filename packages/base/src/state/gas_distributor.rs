use cosmwasm_std::Uint128;
use cw_storage_plus::Map;

pub const TARGET_BALANCES: Map<String, Uint128> = Map::new("target_balances");