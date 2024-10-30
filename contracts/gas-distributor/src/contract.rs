use cosmwasm_std::{
    attr, entry_point, to_json_binary, Addr, Binary, DepsMut, Env, Event, MessageInfo, Order,
    Response, Uint128,
};
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
                target_balance.address.to_string(),
                target_balance.target_balance,
            ));
        });
    Ok(Response::default().add_event(Event::new("instantiate").add_attributes(attrs)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: QueryMsg,
) -> Result<Binary, ContractError> {
    Ok(match msg {
        QueryMsg::TargetBalances() => to_json_binary(&query_target_balances(deps)).unwrap(),
        QueryMsg::TargetBalance { address } => {
            to_json_binary(&query_target_balance(deps, address)).unwrap()
        }
    })
}

fn query_target_balance(deps: DepsMut, address: Addr) -> Uint128 {
    CURRENT_BALANCES
        .load(deps.storage, address.to_string())
        .unwrap()
}

fn query_target_balances(deps: DepsMut) -> Vec<TargetBalance> {
    CURRENT_BALANCES
        .range(deps.storage, None, None, Order::Ascending)
        .into_iter()
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
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}
