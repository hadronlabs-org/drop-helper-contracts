use cosmwasm_schema::write_api;
use drop_helper_contracts_base::msg::gas_distributor::{
    ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg,
};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
        migrate: MigrateMsg
    }
}
