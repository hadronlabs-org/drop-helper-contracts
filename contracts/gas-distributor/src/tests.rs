use crate::contract::{self, query};
use cosmwasm_std::{
    attr, from_json,
    testing::{mock_env, mock_info},
    Addr, Event, Response, Uint128,
};
use drop_helper_contracts_base::{
    error::gas_distributor::ContractError,
    msg::gas_distributor::{InstantiateMsg, QueryMsg, TargetBalance},
    state::gas_distributor::{TargetBalanceUpdateParams, TARGET_BALANCES},
};
use drop_helper_contracts_helpers::testing::mock_dependencies;

#[test]
fn test_instantiate_general() {
    let mut deps = mock_dependencies(&[]);
    let response = contract::instantiate(
        deps.as_mut().into_empty(),
        mock_env(),
        mock_info("sender", &[]),
        InstantiateMsg {
            owner: None,
            initial_target_balances: vec![
                TargetBalance {
                    address: Addr::unchecked("address1"),
                    update_options: TargetBalanceUpdateParams {
                        target_balance: Uint128::from(123_u64),
                        update_value: Some(Uint128::from(2000_u64)),
                    },
                },
                TargetBalance {
                    address: Addr::unchecked("address2"),
                    update_options: TargetBalanceUpdateParams {
                        target_balance: Uint128::from(321_u64),
                        update_value: Some(Uint128::from(1000_u64)),
                    },
                },
            ],
        },
    )
    .unwrap();

    assert_eq!(
        response,
        Response::new().add_event(
            Event::new("crates.io:drop-helper__drop-gas-distributor-instantiate").add_attributes(
                vec![
                    attr("add-target-balance", "address1"),
                    attr("add-target-balance", "address2")
                ]
            )
        )
    );

    let res: Vec<TargetBalance> = from_json(
        query(
            deps.as_ref().into_empty(),
            mock_env(),
            QueryMsg::TargetBalances {},
        )
        .unwrap(),
    )
    .unwrap();

    assert_eq!(
        res,
        vec![
            TargetBalance {
                address: Addr::unchecked("address1"),
                update_options: TargetBalanceUpdateParams {
                    target_balance: Uint128::from(123_u64),
                    update_value: Some(Uint128::from(2000_u64)),
                }
            },
            TargetBalance {
                address: Addr::unchecked("address2"),
                update_options: TargetBalanceUpdateParams {
                    target_balance: Uint128::from(321_u64),
                    update_value: Some(Uint128::from(1000_u64)),
                }
            }
        ]
    );
}

#[test]
fn test_instantiate_custom_owner() {
    let mut deps = mock_dependencies(&[]);
    let response = contract::instantiate(
        deps.as_mut().into_empty(),
        mock_env(),
        mock_info("sender", &[]),
        InstantiateMsg {
            owner: Some(Addr::unchecked("owner")),
            initial_target_balances: vec![
                TargetBalance {
                    address: Addr::unchecked("address1"),
                    update_options: TargetBalanceUpdateParams {
                        target_balance: Uint128::from(123_u64),
                        update_value: Some(Uint128::from(2000_u64)),
                    },
                },
                TargetBalance {
                    address: Addr::unchecked("address2"),
                    update_options: TargetBalanceUpdateParams {
                        target_balance: Uint128::from(321_u64),
                        update_value: Some(Uint128::from(1000_u64)),
                    },
                },
            ],
        },
    )
    .unwrap();

    assert_eq!(
        response,
        Response::new().add_event(
            Event::new("crates.io:drop-helper__drop-gas-distributor-instantiate").add_attributes(
                vec![
                    attr("add-target-balance", "address1"),
                    attr("add-target-balance", "address2")
                ]
            )
        )
    );
    let owner: String =
        from_json(query(deps.as_ref().into_empty(), mock_env(), QueryMsg::Owner {}).unwrap())
            .unwrap();
    assert_eq!(owner, "owner");
}

#[test]
fn test_instantiate_sender_owner() {
    let mut deps = mock_dependencies(&[]);
    let response = contract::instantiate(
        deps.as_mut().into_empty(),
        mock_env(),
        mock_info("sender", &[]),
        InstantiateMsg {
            owner: None,
            initial_target_balances: vec![
                TargetBalance {
                    address: Addr::unchecked("address1"),
                    update_options: TargetBalanceUpdateParams {
                        target_balance: Uint128::from(123_u64),
                        update_value: Some(Uint128::from(2000_u64)),
                    },
                },
                TargetBalance {
                    address: Addr::unchecked("address2"),
                    update_options: TargetBalanceUpdateParams {
                        target_balance: Uint128::from(321_u64),
                        update_value: Some(Uint128::from(1000_u64)),
                    },
                },
            ],
        },
    )
    .unwrap();

    assert_eq!(
        response,
        Response::new().add_event(
            Event::new("crates.io:drop-helper__drop-gas-distributor-instantiate").add_attributes(
                vec![
                    attr("add-target-balance", "address1"),
                    attr("add-target-balance", "address2")
                ]
            )
        )
    );
    let owner: String =
        from_json(query(deps.as_ref().into_empty(), mock_env(), QueryMsg::Owner {}).unwrap())
            .unwrap();
    assert_eq!(owner, "sender");
}

#[test]
fn test_query_owner() {
    let mut deps = mock_dependencies(&[]);
    let deps_mut = deps.as_mut();
    cw_ownable::initialize_owner(deps_mut.storage, deps_mut.api, Some("owner")).unwrap();

    let owner: String =
        from_json(query(deps.as_ref().into_empty(), mock_env(), QueryMsg::Owner {}).unwrap())
            .unwrap();
    assert_eq!(owner, "owner");
}

#[test]
fn test_query_target_balance() {
    let mut deps = mock_dependencies(&[]);
    let deps_mut = deps.as_mut();
    cw_ownable::initialize_owner(deps_mut.storage, deps_mut.api, Some("owner")).unwrap();
    let expected_params = TargetBalanceUpdateParams {
        target_balance: Uint128::from(123_u64),
        update_value: Some(Uint128::from(2000_u64)),
    };
    TARGET_BALANCES
        .save(deps_mut.storage, "address".to_string(), &expected_params)
        .unwrap();
    let response: TargetBalanceUpdateParams = from_json(
        query(
            deps.as_ref().into_empty(),
            mock_env(),
            QueryMsg::TargetBalance {
                address: Addr::unchecked("address"),
            },
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(response, expected_params);
}

#[test]
fn test_query_target_balance_not_exist() {
    let mut deps = mock_dependencies(&[]);
    let deps_mut = deps.as_mut();
    cw_ownable::initialize_owner(deps_mut.storage, deps_mut.api, Some("owner")).unwrap();
    let response: ContractError = query(
        deps.as_ref().into_empty(),
        mock_env(),
        QueryMsg::TargetBalance {
            address: Addr::unchecked("address"),
        },
    )
    .unwrap_err();
    assert_eq!(response, ContractError::UnknownTargetBalance);
}

#[test]
fn test_query_target_balances() {
    let mut deps = mock_dependencies(&[]);
    let deps_mut = deps.as_mut();
    cw_ownable::initialize_owner(deps_mut.storage, deps_mut.api, Some("owner")).unwrap();

    let target_balances = vec![
        TargetBalance {
            address: Addr::unchecked("address1"),
            update_options: TargetBalanceUpdateParams {
                target_balance: Uint128::from(123_u64),
                update_value: Some(Uint128::from(2000_u64)),
            },
        },
        TargetBalance {
            address: Addr::unchecked("address2"),
            update_options: TargetBalanceUpdateParams {
                target_balance: Uint128::from(321_u64),
                update_value: Some(Uint128::from(1000_u64)),
            },
        },
    ];

    target_balances
        .clone()
        .into_iter()
        .for_each(|target_balance| {
            TARGET_BALANCES
                .save(
                    deps_mut.storage,
                    target_balance.address.to_string(),
                    &target_balance.update_options,
                )
                .unwrap();
        });

    let response: Vec<TargetBalance> = from_json(
        query(
            deps.as_ref().into_empty(),
            mock_env(),
            QueryMsg::TargetBalances {},
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(response, target_balances);
}
