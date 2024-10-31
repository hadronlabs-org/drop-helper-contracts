use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use cw_ownable::cw_ownable_execute;

#[cw_serde]
pub struct TargetBalanceUpdateParams {
    pub target_balance: Uint128,
    pub update_value: Uint128,
}

#[cw_serde]
pub struct TargetBalance {
    pub address: Addr,
    pub update_options: TargetBalanceUpdateParams,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Option<Addr>,
    pub initial_target_balances: Vec<TargetBalance>,
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_ownable::cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<TargetBalance>)]
    TargetBalances {},
    #[returns(cosmwasm_std::Uint128)]
    TargetBalance { address: Addr },
}

#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    Distribute {},
    WithdrawTokens {
        recepient: Option<String>,
        amount: Option<Uint128>,
    },
    SetTargetBalances {
        target_balances: Vec<TargetBalance>,
    },
}
