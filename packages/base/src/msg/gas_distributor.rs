use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Deps, Uint128};
use cw_ownable::cw_ownable_execute;

#[cw_serde]
pub struct TargetBalanceUpdateParams {
    pub threshold_balance: Uint128,
    pub target_balance: Uint128,
}

#[cw_serde]
pub struct TargetBalance {
    pub address: String,
    pub update_options: TargetBalanceUpdateParams,
}

impl TargetBalance {
    pub fn validate(&self, deps: Deps) {
        deps.api.addr_validate(&self.address).unwrap();
        assert!(self.update_options.threshold_balance < self.update_options.target_balance)
    }
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
