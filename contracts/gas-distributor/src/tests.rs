use crate::contract::{self, query};
use cosmwasm_std::{
    attr, from_json,
    testing::{mock_env, mock_info},
    Addr, Event, Response, Uint128,
};
use drop_helper_contracts_base::{
    msg::gas_distributor::{InstantiateMsg, QueryMsg, TargetBalance},
    state::gas_distributor::TargetBalanceUpdateParams,
};
use drop_helper_contracts_helpers::testing::mock_dependencies;

#[test]
fn test_instantiate() {
    let mut deps = mock_dependencies(&[]);
    let deps_mut = deps.as_mut();
    let _ = cw_ownable::initialize_owner(deps_mut.storage, deps_mut.api, Some("owner")).unwrap();
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
        Response::new().add_event(Event::new("instantiate").add_attributes(vec![
            attr("add_target_balance", "address1"),
            attr("add_target_balance", "address2")
        ]))
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
