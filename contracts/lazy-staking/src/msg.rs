use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_ownable::cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

#[cw_ownable::cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
pub struct MigrateMsg {}
