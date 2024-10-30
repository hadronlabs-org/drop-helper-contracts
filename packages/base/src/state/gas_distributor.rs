use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use cw_storage_plus::Map;

#[cw_serde]
pub struct TargetBalanceUpdateParams {
    pub target_balance: Uint128,
    pub update_value: Option<Uint128>,
}

pub static UNTRN_DENOM: &'static str = "untrn";
pub const TARGET_BALANCES: Map<String, TargetBalanceUpdateParams> = Map::new("target_balances");
