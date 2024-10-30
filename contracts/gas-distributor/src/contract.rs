use cosmwasm_std::{attr, entry_point, Binary, DepsMut, Env, Event, MessageInfo, Response};
use drop_helper_contracts_base::{
    error::gas_distributor::ContractError,
    msg::gas_distributor::{ExecuteMsg, InstantiateMsg, QueryMsg, TargetBalance},
    state::gas_distributor::CURRENT_BALANCES,
};

const CONTRACT_NAME: &str = concat!("crates.io:drop-staking__", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let mut attrs = vec![attr("action", "instantiate")];
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION).unwrap();
    if msg.owner == None {
        cw_ownable::initialize_owner(deps.storage, deps.api, Some(info.sender.as_str()))?;
    } else {
        cw_ownable::initialize_owner(deps.storage, deps.api, Some(msg.owner.unwrap().as_str()))?;
    }
    msg.initial_target_balances
        .iter()
        .for_each(|target_balance: &TargetBalance| {
            CURRENT_BALANCES
                .save(
                    deps.storage,
                    target_balance.address.to_string(),
                    &target_balance.target_balance,
                )
                .unwrap();
            attrs.push(attr(
                target_balance.address.clone(),
                target_balance.target_balance,
            ));
        });
    Ok(Response::default().add_event(Event::new("instantiate").add_attributes(attrs)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: QueryMsg,
) -> Result<Response, Binary> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}
