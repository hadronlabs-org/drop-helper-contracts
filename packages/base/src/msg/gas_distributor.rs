use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct TargetBalance {
    pub address: Addr,
    pub target_balance: Uint128,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Option<Addr>,
    pub initial_target_balances: Vec<TargetBalance>,
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<TargetBalance>)]
    TargetBalances {},
    #[returns(Uint128)]
    TargetBalance { address: Addr },
}

#[cw_serde]
pub enum ExecuteMsg {
    Distribute {},
    AddTargetBalances { target_balances: Vec<TargetBalance> },
    RemoveTargetBalances { target_balances: Vec<Addr> },
}
