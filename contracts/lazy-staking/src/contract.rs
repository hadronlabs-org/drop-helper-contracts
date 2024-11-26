use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
};
use cosmwasm_std::{
    entry_point, to_json_binary, Attribute, Binary, Deps, DepsMut, Env, MessageInfo, Response,
};
use drop_helper_contracts_helpers::answer::response;
use neutron_sdk::bindings::msg::NeutronMsg;

const CONTRACT_NAME: &str = concat!("crates.io:drop-helper__", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(response(
        "instantiate",
        CONTRACT_NAME,
        Vec::<Attribute>::new(),
    ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Ownership {} => Ok(to_json_binary(
            &cw_ownable::get_ownership(deps.storage)?.owner,
        )?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<NeutronMsg>, ContractError> {
    match msg {
        ExecuteMsg::UpdateOwnership(action) => {
            cw_ownable::update_ownership(deps.into_empty(), &env.block, &info.sender, action)?;
            Ok(response::<(&str, &str), _>(
                "execute-update-ownership",
                CONTRACT_NAME,
                [],
            ))
        }
    }
}
