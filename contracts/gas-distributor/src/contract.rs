use cosmwasm_std::{
    attr, entry_point, to_json_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    Event, MessageInfo, Order, Response, Uint128,
};
use drop_helper_contracts_base::{
    error::gas_distributor::ContractError,
    msg::gas_distributor::{ExecuteMsg, InstantiateMsg, QueryMsg, TargetBalance},
    state::gas_distributor::{TargetBalanceUpdateParams, TARGET_BALANCES},
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
    let mut attrs = vec![];
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION).unwrap();
    if msg.owner.is_none() {
        cw_ownable::initialize_owner(deps.storage, deps.api, Some(info.sender.as_str()))?;
    } else {
        cw_ownable::initialize_owner(deps.storage, deps.api, Some(msg.owner.unwrap().as_str()))?;
    }
    msg.initial_target_balances
        .iter()
        .for_each(|target_balance: &TargetBalance| {
            deps.api
                .addr_validate(target_balance.address.as_str())
                .unwrap();
            TARGET_BALANCES
                .save(
                    deps.storage,
                    target_balance.address.to_string(),
                    &target_balance.update_options,
                )
                .unwrap();
            attrs.push(attr(
                "add_target_balance",
                target_balance.address.to_string(),
            ));
        });
    Ok(Response::default().add_event(Event::new("instantiate").add_attributes(attrs)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    Ok(match msg {
        QueryMsg::TargetBalances {} => to_json_binary(&query_target_balances(deps)).unwrap(),
        QueryMsg::TargetBalance { address } => {
            to_json_binary(&query_target_balance(deps, address)).unwrap()
        }
        QueryMsg::Owner {} => to_json_binary(
            &cw_ownable::get_ownership(deps.storage)?
                .owner
                .unwrap_or(Addr::unchecked(""))
                .to_string(),
        )?,
    })
}

fn query_target_balance(deps: Deps, address: Addr) -> TargetBalanceUpdateParams {
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
                update_options: value,
            }
        })
        .collect()
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
            Ok(Response::new().add_event(Event::new("execute-update-ownership")))
        }
        ExecuteMsg::Distribute {} => execute_distribute(env, deps),
        ExecuteMsg::AddTargetBalances { target_balances } => {
            execute_add_target_balances(deps, info, target_balances)
        }
        ExecuteMsg::RemoveTargetBalances { target_balances } => {
            execute_remove_target_balances(deps, info, target_balances)
        }
        ExecuteMsg::WithdrawTokens { recepient, amount } => {
            execute_withdraw_tokens(deps, info, env, amount, recepient)
        }
    }
}

fn execute_withdraw_tokens(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    amount: Option<Uint128>,
    mut recepient: Option<String>,
) -> Result<Response<NeutronMsg>, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender).unwrap();
    let contract_balance = deps
        .querier
        .query_balance(env.contract.address, "untrn".to_string())
        .unwrap()
        .amount;
    let amount_to_send = match amount {
        Some(a) => a,
        None => contract_balance,
    };
    if amount_to_send > contract_balance {
        return Err(ContractError::InsufficientFunds);
    }
    if recepient.is_none() {
        recepient = Some(info.sender.to_string());
    }
    Ok(Response::new().add_message(CosmosMsg::Bank(BankMsg::Send {
        to_address: recepient.unwrap(),
        amount: vec![Coin {
            denom: "untrn".to_string(),
            amount: amount_to_send,
        }],
    })))
}

fn execute_add_target_balances(
    deps: DepsMut,
    info: MessageInfo,
    target_balances: Vec<TargetBalance>,
) -> Result<Response<NeutronMsg>, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender).unwrap();
    let mut attrs = vec![];
    target_balances.into_iter().for_each(|target_balance| {
        deps.api
            .addr_validate(target_balance.address.as_str())
            .unwrap();
        TARGET_BALANCES
            .save(
                deps.storage,
                target_balance.address.to_string(),
                &target_balance.update_options,
            )
            .unwrap();
        attrs.push(attr("add_target_balance", target_balance.address));
    });
    Ok(Response::new().add_event(Event::new("execute_add_target_balances").add_attributes(attrs)))
}

fn execute_remove_target_balances(
    deps: DepsMut,
    info: MessageInfo,
    target_balances: Vec<Addr>,
) -> Result<Response<NeutronMsg>, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender).unwrap();
    let mut attrs = vec![];
    for addr in target_balances {
        if TARGET_BALANCES.has(deps.storage, addr.to_string()) {
            TARGET_BALANCES.remove(deps.storage, addr.to_string());
            attrs.push(attr("remove_target_balance", addr.to_string()));
        } else {
            return Err(ContractError::UnknownTargetBalance {});
        }
    }
    Ok(Response::new()
        .add_event(Event::new("execute_remove_target_balances").add_attributes(attrs)))
}

fn execute_distribute(env: Env, deps: DepsMut) -> Result<Response<NeutronMsg>, ContractError> {
    let mut attrs = vec![];
    let mut messages = vec![];
    let mut total_funds_required = 0u128;
    for target_balance in TARGET_BALANCES.range(deps.storage, None, None, Order::Ascending) {
        let (address, update_options) = target_balance.unwrap();
        let current_balance = deps
            .querier
            .query_balance(address.clone(), "untrn".to_string())?
            .amount;
        if current_balance < update_options.target_balance {
            let abs_delta = current_balance.abs_diff(update_options.target_balance);
            let funds_to_send = match update_options.update_value {
                Some(_) => abs_delta + update_options.update_value.unwrap(),
                None => abs_delta,
            };
            messages.push(CosmosMsg::Bank(BankMsg::Send {
                to_address: address.clone(),
                amount: vec![Coin {
                    denom: "untrn".to_string(),
                    amount: funds_to_send,
                }],
            }));
            attrs.push(attr(address, funds_to_send));
            total_funds_required += funds_to_send.u128();
        }
    }
    let contract_balance = deps
        .querier
        .query_balance(env.contract.address, "untrn".to_string())?
        .amount
        .u128();
    if total_funds_required > contract_balance {
        return Err(ContractError::InsufficientFunds {});
    }
    Ok(Response::new()
        .add_event(Event::new("execute_distribute").add_attributes(attrs))
        .add_messages(messages))
}
