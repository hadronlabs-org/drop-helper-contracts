use cosmwasm_std::{
    attr, entry_point, to_json_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    Event, MessageInfo, Order, Response, Uint128,
};
use drop_helper_contracts_base::{
    error::gas_distributor::ContractError,
    msg::gas_distributor::{ExecuteMsg, InstantiateMsg, QueryMsg, TargetBalance},
    state::gas_distributor::TARGET_BALANCES,
};
use neutron_sdk::bindings::msg::NeutronMsg;

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
    if msg.owner.is_none() {
        cw_ownable::initialize_owner(deps.storage, deps.api, Some(info.sender.as_str()))?;
    } else {
        cw_ownable::initialize_owner(deps.storage, deps.api, Some(msg.owner.unwrap().as_str()))?;
    }
    msg.initial_target_balances
        .iter()
        .for_each(|target_balance: &TargetBalance| {
            TARGET_BALANCES
                .save(
                    deps.storage,
                    target_balance.address.to_string(),
                    &target_balance.target_balance,
                )
                .unwrap();
            attrs.push(attr(
                target_balance.address.to_string(),
                target_balance.target_balance,
            ));
        });
    Ok(Response::default().add_event(Event::new("instantiate").add_attributes(attrs)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    Ok(match msg {
        QueryMsg::TargetBalances() => to_json_binary(&query_target_balances(deps)).unwrap(),
        QueryMsg::TargetBalance { address } => {
            to_json_binary(&query_target_balance(deps, address)).unwrap()
        }
    })
}

fn query_target_balance(deps: Deps, address: Addr) -> Uint128 {
    TARGET_BALANCES
        .load(deps.storage, address.to_string())
        .unwrap()
}

fn query_target_balances(deps: Deps) -> Vec<TargetBalance> {
    TARGET_BALANCES
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| {
            let (key, value) = item.unwrap();
            TargetBalance {
                address: Addr::unchecked(key),
                target_balance: value,
            }
        })
        .collect()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<NeutronMsg>, ContractError> {
    match msg {
        ExecuteMsg::Distribute {} => execute_distribute(deps),
    }
}

fn execute_distribute(deps: DepsMut) -> Result<Response<NeutronMsg>, ContractError> {
    let mut attrs = vec![];
    let mut messages = vec![];
    for item in TARGET_BALANCES.range(deps.storage, None, None, Order::Ascending) {
        let (address, target_balance) = item.unwrap();
        let current_balance = deps
            .querier
            .query_balance(address.clone(), "untrn".to_string())?
            .amount;
        let delta = current_balance - target_balance;
        if delta.lt(&Uint128::zero()) {
            let abs_delta = delta.abs_diff(Uint128::zero());
            messages.push(CosmosMsg::Bank(BankMsg::Send {
                to_address: address.clone(),
                amount: vec![Coin {
                    denom: "untrn".to_string(),
                    amount: abs_delta,
                }],
            }));
            attrs.push(attr(address, abs_delta))
        }
    }
    Ok(Response::new()
        .add_event(Event::new("execute_distribute").add_attributes(attrs))
        .add_messages(messages))
}
