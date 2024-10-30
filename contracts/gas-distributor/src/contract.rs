use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response};
use drop_helper_contracts_base::{
    error::gas_distributor::ContractError, msg::gas_distributor::InstantiateMsg,
};

const CONTRACT_NAME: &str = concat!("crates.io:drop-staking__", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION).unwrap();
    unimplemented!()
}
