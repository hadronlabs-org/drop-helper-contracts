use crate::contract::{execute, instantiate, query};
use cosmwasm_std::{
    attr, from_json,
    testing::{mock_env, mock_info},
    Addr, BalanceResponse, BankMsg, Event, Response, SubMsg, Uint128,
};
use drop_helper_contracts_base::{
    error::gas_distributor::ContractError,
    msg::gas_distributor::{
        ExecuteMsg, InstantiateMsg, QueryMsg, TargetBalance, TargetBalanceUpdateParams,
    },
    state::gas_distributor::{TARGET_BALANCES, UNTRN_DENOM},
};
use drop_helper_contracts_helpers::testing::mock_dependencies;

#[test]
fn test_instantiate_general() {
    let mut deps = mock_dependencies(&[]);
    let response = instantiate(
        deps.as_mut().into_empty(),
        mock_env(),
        mock_info("sender", &[]),
        InstantiateMsg {
            owner: None,
            initial_target_balances: vec![
                TargetBalance {
                    address: "address1".to_string(),
                    update_options: TargetBalanceUpdateParams {
                        target_balance: Uint128::from(2000_u64),
                        threshold_balance: Uint128::from(123_u64),
                    },
                },
                TargetBalance {
                    address: "address2".to_string(),
                    update_options: TargetBalanceUpdateParams {
                        target_balance: Uint128::from(1000_u64),
                        threshold_balance: Uint128::from(321_u64),
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
                address: "address1".to_string(),
                update_options: TargetBalanceUpdateParams {
                    target_balance: Uint128::from(2000_u64),
                    threshold_balance: Uint128::from(123_u64),
                }
            },
            TargetBalance {
                address: "address2".to_string(),
                update_options: TargetBalanceUpdateParams {
                    target_balance: Uint128::from(1000_u64),
                    threshold_balance: Uint128::from(321_u64),
                }
            }
        ]
    );
}

#[test]
fn test_instantiate_custom_owner() {
    let mut deps = mock_dependencies(&[]);
    let response = instantiate(
        deps.as_mut().into_empty(),
        mock_env(),
        mock_info("sender", &[]),
        InstantiateMsg {
            owner: Some(Addr::unchecked("owner")),
            initial_target_balances: vec![
                TargetBalance {
                    address: "address1".to_string(),
                    update_options: TargetBalanceUpdateParams {
                        target_balance: Uint128::from(2000_u64),
                        threshold_balance: Uint128::from(123_u64),
                    },
                },
                TargetBalance {
                    address: "address2".to_string(),
                    update_options: TargetBalanceUpdateParams {
                        target_balance: Uint128::from(1000_u64),
                        threshold_balance: Uint128::from(321_u64),
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
    let owner: String = from_json(
        query(
            deps.as_ref().into_empty(),
            mock_env(),
            QueryMsg::Ownership {},
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(owner, "owner");
}

#[test]
fn test_instantiate_sender_owner() {
    let mut deps = mock_dependencies(&[]);
    let response = instantiate(
        deps.as_mut().into_empty(),
        mock_env(),
        mock_info("sender", &[]),
        InstantiateMsg {
            owner: None,
            initial_target_balances: vec![
                TargetBalance {
                    address: "address1".to_string(),
                    update_options: TargetBalanceUpdateParams {
                        target_balance: Uint128::from(2000_u64),
                        threshold_balance: Uint128::from(123_u64),
                    },
                },
                TargetBalance {
                    address: "address2".to_string(),
                    update_options: TargetBalanceUpdateParams {
                        target_balance: Uint128::from(1000_u64),
                        threshold_balance: Uint128::from(321_u64),
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
    let owner: String = from_json(
        query(
            deps.as_ref().into_empty(),
            mock_env(),
            QueryMsg::Ownership {},
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(owner, "sender");
}

#[test]
fn test_query_owner() {
    let mut deps = mock_dependencies(&[]);
    let deps_mut = deps.as_mut();
    cw_ownable::initialize_owner(deps_mut.storage, deps_mut.api, Some("owner")).unwrap();

    let owner: String = from_json(
        query(
            deps.as_ref().into_empty(),
            mock_env(),
            QueryMsg::Ownership {},
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(owner, "owner");
}

#[test]
fn test_query_target_balance() {
    let mut deps = mock_dependencies(&[]);
    let deps_mut = deps.as_mut();
    cw_ownable::initialize_owner(deps_mut.storage, deps_mut.api, Some("owner")).unwrap();
    let expected_params = TargetBalance {
        address: "address".to_string(),
        update_options: TargetBalanceUpdateParams {
            target_balance: Uint128::from(2000_u64),
            threshold_balance: Uint128::from(123_u64),
        },
    };
    TARGET_BALANCES
        .save(deps_mut.storage, &vec![expected_params.clone()])
        .unwrap();
    let response: TargetBalance = from_json(
        query(
            deps.as_ref().into_empty(),
            mock_env(),
            QueryMsg::TargetBalance {
                address: Addr::unchecked("address".to_string()),
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
    TARGET_BALANCES
        .save(deps.as_mut().storage, &vec![])
        .unwrap();
    let response: ContractError = query(
        deps.as_ref().into_empty(),
        mock_env(),
        QueryMsg::TargetBalance {
            address: Addr::unchecked("address".to_string()),
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
            address: "address1".to_string(),
            update_options: TargetBalanceUpdateParams {
                target_balance: Uint128::from(2000_u64),
                threshold_balance: Uint128::from(123_u64),
            },
        },
        TargetBalance {
            address: "address2".to_string(),
            update_options: TargetBalanceUpdateParams {
                target_balance: Uint128::from(1000_u64),
                threshold_balance: Uint128::from(321_u64),
            },
        },
    ];

    TARGET_BALANCES
        .save(deps_mut.storage, &target_balances)
        .unwrap();

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

#[test]
fn test_execute_set_target_balances_unauthorized() {
    let mut deps = mock_dependencies(&[]);
    let deps_mut = deps.as_mut();
    cw_ownable::initialize_owner(deps_mut.storage, deps_mut.api, Some("owner")).unwrap();
    let execute_res = execute(
        deps_mut.into_empty(),
        mock_env(),
        mock_info("somebody", &[]),
        ExecuteMsg::SetTargetBalances {
            target_balances: vec![],
        },
    )
    .unwrap_err();
    assert_eq!(
        execute_res,
        ContractError::OwnershipError(cw_ownable::OwnershipError::NotOwner)
    );
}

#[test]
fn test_execute_set_target_balances() {
    let mut deps = mock_dependencies(&[]);
    let deps_mut = deps.as_mut();
    cw_ownable::initialize_owner(deps_mut.storage, deps_mut.api, Some("owner")).unwrap();
    let expected_target_balances = vec![
        TargetBalance {
            address: "address1".to_string(),
            update_options: TargetBalanceUpdateParams {
                target_balance: Uint128::from(2000_u64),
                threshold_balance: Uint128::from(123_u64),
            },
        },
        TargetBalance {
            address: "address2".to_string(),
            update_options: TargetBalanceUpdateParams {
                target_balance: Uint128::from(1000_u64),
                threshold_balance: Uint128::from(321_u64),
            },
        },
    ];
    let execute_res = execute(
        deps_mut.into_empty(),
        mock_env(),
        mock_info("owner", &[]),
        ExecuteMsg::SetTargetBalances {
            target_balances: expected_target_balances.clone(),
        },
    )
    .unwrap();
    assert_eq!(
        execute_res,
        Response::new().add_event(
            Event::new("crates.io:drop-helper__drop-gas-distributor-execute-set-target-balances")
                .add_attributes(vec![
                    attr("set-target-balance", "address1"),
                    attr("set-target-balance", "address2")
                ])
        )
    );
    let target_balances_list = TARGET_BALANCES.load(&deps.storage).unwrap();
    assert_eq!(target_balances_list, expected_target_balances)
}

#[test]
fn test_execute_withdraw_tokens_unauthorized() {
    let mut deps = mock_dependencies(&[]);
    let deps_mut = deps.as_mut();
    cw_ownable::initialize_owner(deps_mut.storage, deps_mut.api, Some("owner")).unwrap();
    let execute_res = execute(
        deps_mut.into_empty(),
        mock_env(),
        mock_info("somebody", &[]),
        ExecuteMsg::WithdrawTokens {
            recepient: None,
            amount: None,
        },
    )
    .unwrap_err();
    assert_eq!(
        execute_res,
        ContractError::OwnershipError(cw_ownable::OwnershipError::NotOwner)
    );
}

#[test]
fn test_execute_withdraw_tokens_unauthorized_custom_recepient() {
    let mut deps = mock_dependencies(&[]);
    let deps_mut = deps.as_mut();
    cw_ownable::initialize_owner(deps_mut.storage, deps_mut.api, Some("owner")).unwrap();
    let execute_res = execute(
        deps_mut.into_empty(),
        mock_env(),
        mock_info("owner", &[]),
        ExecuteMsg::WithdrawTokens {
            recepient: Some("recepient".to_string()),
            amount: None,
        },
    )
    .unwrap();
    assert_eq!(
        execute_res,
        Response::new()
            .add_submessage(SubMsg {
                id: 0,
                msg: cosmwasm_std::CosmosMsg::Bank(BankMsg::Send {
                    to_address: "recepient".to_string(),
                    amount: vec![cosmwasm_std::Coin {
                        denom: "untrn".to_string(),
                        amount: Uint128::from(0_u128)
                    }]
                }),
                gas_limit: None,
                reply_on: cosmwasm_std::ReplyOn::Never
            })
            .add_event(Event::new(
                "crates.io:drop-helper__drop-gas-distributor-execute-withdraw-tokens"
            ))
    );
}

#[test]
fn test_execute_withdraw_tokens_unauthorized_custom_amount_insufficient_funds() {
    let mut deps = mock_dependencies(&[]);
    let deps_mut = deps.as_mut();
    cw_ownable::initialize_owner(deps_mut.storage, deps_mut.api, Some("owner")).unwrap();
    let execute_res = execute(
        deps_mut.into_empty(),
        mock_env(),
        mock_info("owner", &[]),
        ExecuteMsg::WithdrawTokens {
            recepient: Some("recepient".to_string()),
            amount: Some(Uint128::from(123_u128)),
        },
    )
    .unwrap_err();
    assert_eq!(execute_res, ContractError::InsufficientFunds {});
}

#[test]
fn test_execute_withdraw_tokens_unauthorized_custom_amount() {
    let mut deps = mock_dependencies(&[cosmwasm_std::Coin {
        denom: "untrn".to_string(),
        amount: Uint128::from(123_u128),
    }]);
    let deps_mut = deps.as_mut();
    cw_ownable::initialize_owner(deps_mut.storage, deps_mut.api, Some("owner")).unwrap();
    let execute_res = execute(
        deps_mut.into_empty(),
        mock_env(),
        mock_info("owner", &[]),
        ExecuteMsg::WithdrawTokens {
            recepient: None,
            amount: Some(Uint128::from(123_u128)),
        },
    )
    .unwrap();
    assert_eq!(
        execute_res,
        Response::new()
            .add_submessage(SubMsg {
                id: 0,
                msg: cosmwasm_std::CosmosMsg::Bank(BankMsg::Send {
                    to_address: "owner".to_string(),
                    amount: vec![cosmwasm_std::Coin {
                        denom: "untrn".to_string(),
                        amount: Uint128::from(123_u128)
                    }]
                }),
                gas_limit: None,
                reply_on: cosmwasm_std::ReplyOn::Never
            })
            .add_event(Event::new(
                "crates.io:drop-helper__drop-gas-distributor-execute-withdraw-tokens"
            ))
    );
}

#[test]
fn test_execute_withdraw_tokens_unauthorized_custom_amount_and_recepient() {
    let mut deps = mock_dependencies(&[cosmwasm_std::Coin {
        denom: "untrn".to_string(),
        amount: Uint128::from(123_u128),
    }]);
    let deps_mut = deps.as_mut();
    cw_ownable::initialize_owner(deps_mut.storage, deps_mut.api, Some("owner")).unwrap();
    let execute_res = execute(
        deps_mut.into_empty(),
        mock_env(),
        mock_info("owner", &[]),
        ExecuteMsg::WithdrawTokens {
            recepient: Some("recepient".to_string()),
            amount: Some(Uint128::from(123_u128)),
        },
    )
    .unwrap();
    assert_eq!(
        execute_res,
        Response::new()
            .add_submessage(SubMsg {
                id: 0,
                msg: cosmwasm_std::CosmosMsg::Bank(BankMsg::Send {
                    to_address: "recepient".to_string(),
                    amount: vec![cosmwasm_std::Coin {
                        denom: "untrn".to_string(),
                        amount: Uint128::from(123_u128)
                    }]
                }),
                gas_limit: None,
                reply_on: cosmwasm_std::ReplyOn::Never
            })
            .add_event(Event::new(
                "crates.io:drop-helper__drop-gas-distributor-execute-withdraw-tokens"
            ))
    );
}

#[test]
fn test_distribute_2_addresses() {
    let mut deps = mock_dependencies(&[cosmwasm_std::Coin {
        denom: "untrn".to_string(),
        amount: Uint128::from(1000_u128),
    }]);
    let expected_params = TargetBalanceUpdateParams {
        target_balance: Uint128::from(100_u64),
        threshold_balance: Uint128::from(10_u64),
    };
    TARGET_BALANCES
        .save(
            deps.as_mut().storage,
            &vec![
                TargetBalance {
                    address: "address1".to_string(),
                    update_options: expected_params.clone(),
                },
                TargetBalance {
                    address: "address2".to_string(),
                    update_options: expected_params,
                },
            ],
        )
        .unwrap();
    deps.querier.add_bank_query_response(
        "address2".to_string(),
        BalanceResponse {
            amount: cosmwasm_std::Coin {
                denom: UNTRN_DENOM.to_string(),
                amount: Uint128::from(9_u128),
            },
        },
    );
    let execute_res = execute(
        deps.as_mut().into_empty(),
        mock_env(),
        mock_info("owner", &[]),
        ExecuteMsg::Distribute {},
    )
    .unwrap();

    assert_eq!(
        execute_res,
        Response::new()
            .add_submessages(vec![
                SubMsg {
                    id: 0_u64,
                    msg: cosmwasm_std::CosmosMsg::Bank(BankMsg::Send {
                        to_address: "address1".to_string(),
                        amount: vec![cosmwasm_std::Coin {
                            denom: "untrn".to_string(),
                            amount: Uint128::from(100_u128)
                        }]
                    }),
                    gas_limit: None,
                    reply_on: cosmwasm_std::ReplyOn::Never
                },
                SubMsg {
                    id: 0_u64,
                    msg: cosmwasm_std::CosmosMsg::Bank(BankMsg::Send {
                        to_address: "address2".to_string(),
                        amount: vec![cosmwasm_std::Coin {
                            denom: "untrn".to_string(),
                            amount: Uint128::from(91_u128)
                        }]
                    }),
                    gas_limit: None,
                    reply_on: cosmwasm_std::ReplyOn::Never
                }
            ])
            .add_event(
                Event::new("crates.io:drop-helper__drop-gas-distributor-execute-distribute")
                    .add_attributes(vec![attr("address1", "100"), attr("address2", "91")])
            )
    );
}

#[test]
fn test_distribute_1_address() {
    let mut deps = mock_dependencies(&[cosmwasm_std::Coin {
        denom: "untrn".to_string(),
        amount: Uint128::from(150_u128),
    }]);
    let expected_params = TargetBalanceUpdateParams {
        target_balance: Uint128::from(100_u64),
        threshold_balance: Uint128::from(10_u64),
    };
    TARGET_BALANCES
        .save(
            deps.as_mut().storage,
            &vec![
                TargetBalance {
                    address: "address1".to_string(),
                    update_options: expected_params.clone(),
                },
                TargetBalance {
                    address: "address2".to_string(),
                    update_options: expected_params,
                },
            ],
        )
        .unwrap();
    deps.querier.add_bank_query_response(
        "address1".to_string(),
        BalanceResponse {
            amount: cosmwasm_std::Coin {
                denom: UNTRN_DENOM.to_string(),
                amount: Uint128::from(9_u128),
            },
        },
    );
    let execute_res = execute(
        deps.as_mut().into_empty(),
        mock_env(),
        mock_info("owner", &[]),
        ExecuteMsg::Distribute {},
    )
    .unwrap();

    assert_eq!(
        execute_res,
        Response::new()
            .add_submessages(vec![SubMsg {
                id: 0_u64,
                msg: cosmwasm_std::CosmosMsg::Bank(BankMsg::Send {
                    to_address: "address1".to_string(),
                    amount: vec![cosmwasm_std::Coin {
                        denom: "untrn".to_string(),
                        amount: Uint128::from(91_u128)
                    }]
                }),
                gas_limit: None,
                reply_on: cosmwasm_std::ReplyOn::Never
            }])
            .add_event(
                Event::new("crates.io:drop-helper__drop-gas-distributor-execute-distribute")
                    .add_attributes(vec![attr("address1", "91")])
            )
    );
}

#[test]
fn test_distribute_nobody() {
    let mut deps = mock_dependencies(&[cosmwasm_std::Coin {
        denom: "untrn".to_string(),
        amount: Uint128::from(10_u128),
    }]);
    let expected_params = TargetBalanceUpdateParams {
        target_balance: Uint128::from(100_u64),
        threshold_balance: Uint128::from(10_u64),
    };
    TARGET_BALANCES
        .save(
            deps.as_mut().storage,
            &vec![
                TargetBalance {
                    address: "address1".to_string(),
                    update_options: expected_params.clone(),
                },
                TargetBalance {
                    address: "address2".to_string(),
                    update_options: expected_params,
                },
            ],
        )
        .unwrap();
    deps.querier.add_bank_query_response(
        "address2".to_string(),
        BalanceResponse {
            amount: cosmwasm_std::Coin {
                denom: UNTRN_DENOM.to_string(),
                amount: Uint128::from(0_u128),
            },
        },
    );
    let execute_res = execute(
        deps.as_mut().into_empty(),
        mock_env(),
        mock_info("owner", &[]),
        ExecuteMsg::Distribute {},
    )
    .unwrap();

    assert_eq!(
        execute_res,
        Response::new().add_event(Event::new(
            "crates.io:drop-helper__drop-gas-distributor-execute-distribute"
        ))
    );
}

#[test]
fn test_query_ownership() {
    let mut deps = mock_dependencies(&[]);
    let deps_mut = deps.as_mut();
    cw_ownable::initialize_owner(deps_mut.storage, deps_mut.api, Some("owner")).unwrap();
    let query_res: String = from_json(
        query(
            deps.as_ref().into_empty(),
            mock_env(),
            QueryMsg::Ownership {},
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(query_res, "owner".to_string());
}

#[test]
fn test_transfer_ownership() {
    let mut deps = mock_dependencies(&[]);
    let deps_mut = deps.as_mut();
    cw_ownable::initialize_owner(deps_mut.storage, deps_mut.api, Some("owner")).unwrap();
    execute(
        deps.as_mut().into_empty(),
        mock_env(),
        mock_info("owner", &[]),
        ExecuteMsg::UpdateOwnership(cw_ownable::Action::TransferOwnership {
            new_owner: "new_owner".to_string(),
            expiry: Some(cw_ownable::Expiration::Never {}),
        }),
    )
    .unwrap();
    execute(
        deps.as_mut().into_empty(),
        mock_env(),
        mock_info("new_owner", &[]),
        ExecuteMsg::UpdateOwnership(cw_ownable::Action::AcceptOwnership {}),
    )
    .unwrap();
    let query_res: String = from_json(
        query(
            deps.as_ref().into_empty(),
            mock_env(),
            QueryMsg::Ownership {},
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(query_res, "new_owner".to_string());
}
