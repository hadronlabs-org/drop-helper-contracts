use cosmwasm_std::{
    attr, entry_point, to_json_binary, Addr, Attribute, BankMsg, Binary, Coin, CosmosMsg, Deps,
    DepsMut, Env, MessageInfo, Order, Response, Uint128,
};
use drop_helper_contracts_base::{
    error::gas_distributor::ContractError,
    msg::gas_distributor::{ExecuteMsg, InstantiateMsg, QueryMsg, TargetBalance},
    state::gas_distributor::{TARGET_BALANCES, UNTRN_DENOM},
};
use drop_helper_contracts_helpers::answer::response;
use neutron_sdk::bindings::msg::NeutronMsg;

const CONTRACT_NAME: &str = concat!("crates.io:drop-helper__", env!("CARGO_PKG_NAME"));
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
    let owner = deps
        .api
        .addr_validate(msg.owner.unwrap_or(info.sender).as_str())
        .unwrap();
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(owner.as_str()))?;
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
                    target_balance,
                )
                .unwrap();
            attrs.push(attr(
                "add-target-balance",
                target_balance.address.to_string(),
            ));
        });
    Ok(response("instantiate", CONTRACT_NAME, attrs))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    Ok(match msg {
        QueryMsg::TargetBalances {} => query_target_balances(deps)?,
        QueryMsg::TargetBalance { address } => query_target_balance(deps, address)?,
        QueryMsg::Ownership {} => to_json_binary(&cw_ownable::get_ownership(deps.storage)?.owner)?,
    })
}

fn query_target_balance(deps: Deps, address: Addr) -> Result<Binary, ContractError> {
    match TARGET_BALANCES.load(deps.storage, address.to_string()) {
        Ok(value) => Ok(to_json_binary(&value).unwrap()),
        Err(_) => Err(ContractError::UnknownTargetBalance {}),
    }
}

fn query_target_balances(deps: Deps) -> Result<Binary, ContractError> {
    Ok(to_json_binary(
        &TARGET_BALANCES
            .range(deps.storage, None, None, Order::Ascending)
            .map(
                |item: Result<(String, TargetBalance), cosmwasm_std::StdError>| {
                    let (_, target_balance) = item.unwrap();
                    target_balance
                },
            )
            .collect::<Vec<TargetBalance>>(),
    )
    .unwrap())
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
    recepient: Option<String>,
) -> Result<Response<NeutronMsg>, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender)?;
    let contract_balance = deps
        .querier
        .query_balance(env.contract.address, UNTRN_DENOM.to_string())
        .unwrap()
        .amount;
    let amount_to_send = amount.unwrap_or(contract_balance);
    if amount_to_send > contract_balance {
        return Err(ContractError::InsufficientFunds);
    }
    let recepient_to_send = recepient.unwrap_or(info.sender.to_string());
    Ok(response(
        "execute-withdraw-tokens",
        CONTRACT_NAME,
        Vec::<Attribute>::new(),
    )
    .add_message(CosmosMsg::Bank(BankMsg::Send {
        to_address: recepient_to_send,
        amount: vec![Coin {
            denom: UNTRN_DENOM.to_string(),
            amount: amount_to_send,
        }],
    })))
}

fn execute_add_target_balances(
    deps: DepsMut,
    info: MessageInfo,
    target_balances: Vec<TargetBalance>,
) -> Result<Response<NeutronMsg>, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender)?;
    let mut attrs = vec![];
    target_balances.into_iter().for_each(|target_balance| {
        deps.api
            .addr_validate(target_balance.address.as_str())
            .unwrap();
        TARGET_BALANCES
            .save(
                deps.storage,
                target_balance.address.to_string(),
                &target_balance,
            )
            .unwrap();
        attrs.push(attr("add-target-balance", target_balance.address));
    });
    Ok(response(
        "execute-add-target-balances",
        CONTRACT_NAME,
        attrs,
    ))
}

fn execute_remove_target_balances(
    deps: DepsMut,
    info: MessageInfo,
    target_balances: Vec<Addr>,
) -> Result<Response<NeutronMsg>, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender)?;
    let mut attrs = vec![];
    for addr in target_balances {
        if TARGET_BALANCES.has(deps.storage, addr.to_string()) {
            TARGET_BALANCES.remove(deps.storage, addr.to_string());
            attrs.push(attr("remove-target-balance", addr.to_string()));
        } else {
            return Err(ContractError::UnknownTargetBalance {});
        }
    }
    Ok(response(
        "execute-remove-target-balances",
        CONTRACT_NAME,
        attrs,
    ))
}

fn execute_distribute(env: Env, deps: DepsMut) -> Result<Response<NeutronMsg>, ContractError> {
    let mut attrs = vec![];
    let mut messages = vec![];
    let mut contract_balance = deps
        .querier
        .query_balance(env.contract.address, UNTRN_DENOM.to_string())?
        .amount;

    for item in TARGET_BALANCES.range(deps.storage, None, None, Order::Ascending) {
        let (address, target_balance) = item.unwrap();
        let current_balance = deps
            .querier
            .query_balance(address.clone(), UNTRN_DENOM.to_string())?
            .amount;
        if current_balance < target_balance.update_options.target_balance {
            let abs_delta = current_balance.abs_diff(target_balance.update_options.target_balance);
            let funds_to_send = match target_balance.update_options.update_value {
                Some(_) => abs_delta + target_balance.update_options.update_value.unwrap(),
                None => abs_delta,
            };
            if contract_balance.checked_sub(funds_to_send).is_ok() {
                messages.push(CosmosMsg::Bank(BankMsg::Send {
                    to_address: address.clone(),
                    amount: vec![Coin {
                        denom: UNTRN_DENOM.to_string(),
                        amount: funds_to_send,
                    }],
                }));
                contract_balance = contract_balance.abs_diff(funds_to_send);
                attrs.push(attr(address, funds_to_send));
            } else {
                break;
            }
        }
    }
    Ok(response("execute-distribute", CONTRACT_NAME, attrs).add_messages(messages))
}
