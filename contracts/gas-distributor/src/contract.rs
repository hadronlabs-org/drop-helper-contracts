use cosmwasm_std::{
    attr, ensure, entry_point, to_json_binary, Addr, Attribute, BankMsg, Binary, Coin, CosmosMsg,
    Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
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
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let owner = deps
        .api
        .addr_validate(msg.owner.unwrap_or(info.sender).as_str())?;
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(owner.as_str()))?;
    for target_balance in msg.initial_target_balances.clone() {
        deps.api.addr_validate(target_balance.address.as_str())?;
        attrs.push(attr(
            "add-target-balance",
            target_balance.address.to_string(),
        ));
    }
    msg.initial_target_balances
        .iter()
        .for_each(|target_balance| target_balance.validate(deps.as_ref()));
    TARGET_BALANCES.save(deps.storage, &msg.initial_target_balances)?;
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
    Ok(to_json_binary(
        &TARGET_BALANCES
            .load(deps.storage)?
            .into_iter()
            .find(|target_balance| target_balance.address == address)
            .ok_or(ContractError::UnknownTargetBalance)?,
    )?)
}

fn query_target_balances(deps: Deps) -> Result<Binary, ContractError> {
    Ok(to_json_binary(&TARGET_BALANCES.load(deps.storage)?)?)
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
        ExecuteMsg::SetTargetBalances { target_balances } => {
            execute_set_target_balances(deps, info, target_balances)
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
        .query_balance(env.contract.address, UNTRN_DENOM.to_string())?
        .amount;
    let amount_to_send = amount.unwrap_or(contract_balance);
    ensure!(
        amount_to_send <= contract_balance,
        ContractError::InsufficientFunds
    );
    let recepient = recepient.unwrap_or(info.sender.to_string());
    Ok(response(
        "execute-withdraw-tokens",
        CONTRACT_NAME,
        Vec::<Attribute>::new(),
    )
    .add_message(CosmosMsg::Bank(BankMsg::Send {
        to_address: recepient,
        amount: vec![Coin {
            denom: UNTRN_DENOM.to_string(),
            amount: amount_to_send,
        }],
    })))
}

fn execute_set_target_balances(
    deps: DepsMut,
    info: MessageInfo,
    target_balances: Vec<TargetBalance>,
) -> Result<Response<NeutronMsg>, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender)?;
    let attrs = target_balances
        .clone()
        .into_iter()
        .map(|target_balance| {
            deps.api.addr_validate(target_balance.address.as_str())?;
            Ok(attr("set-target-balance", target_balance.address))
        })
        .collect::<StdResult<Vec<_>>>()?;
    target_balances
        .iter()
        .for_each(|target_balance| target_balance.validate(deps.as_ref()));
    TARGET_BALANCES.save(deps.storage, &target_balances)?;
    Ok(response(
        "execute-set-target-balances",
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
    for target_balance in TARGET_BALANCES.load(deps.storage)? {
        let current_balance = deps
            .querier
            .query_balance(target_balance.address.clone(), UNTRN_DENOM.to_string())?
            .amount;
        if current_balance < target_balance.update_options.threshold_balance {
            let funds_to_send = target_balance.update_options.target_balance - current_balance;
            if contract_balance.checked_sub(funds_to_send).is_ok() {
                messages.push(CosmosMsg::Bank(BankMsg::Send {
                    to_address: target_balance.address.to_string(),
                    amount: vec![Coin {
                        denom: UNTRN_DENOM.to_string(),
                        amount: funds_to_send,
                    }],
                }));
                contract_balance = contract_balance.abs_diff(funds_to_send);
                attrs.push(attr(target_balance.address.to_string(), funds_to_send));
            }
        }
    }
    Ok(response("execute-distribute", CONTRACT_NAME, attrs).add_messages(messages))
}
