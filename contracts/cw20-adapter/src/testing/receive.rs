use crate::testing::utils::{assert_execute_error, instantiate, instantiate_cw20, mint_cw20, register_contract, send_cw20, store_code_with_path};

use cosmwasm_std::{coins, Uint128};
use injective_test_tube::{Account, Module, Wasm};
use injective_testing::test_tube::utils::store_code;
use testenv::utils::Setup;

use super::utils::query_balance_cw20;

#[test]
fn test_receive() {
    let env = Setup::new();

    let wasm = Wasm::new(&env.app);

    let adaptor_code_id = store_code(&wasm, &env.owner, "cw20_adapter".to_string());
    let base_code_id = store_code_with_path(
        &wasm,
        &env.owner,
        "../../contracts/cw20-adapter/src/testing/test_artifacts/cw20_base.wasm".to_string(),
    );

    let adaptor_addr = instantiate(&env.app, &env.owner, adaptor_code_id, "cw20-adapter");
    let cw20_addr = instantiate_cw20(&env.app, &env.owner, base_code_id, "cw20-base", "cw20-mofo");

    let deposit = coins(10_000_000_000_000_000_000u128, "inj");
    register_contract(&env.app, &env.owner, &adaptor_addr, &cw20_addr, deposit).unwrap();

    let mint_amount = Uint128::from(2_000_000u128);
    mint_cw20(&env.app, &env.owner, &cw20_addr, mint_amount, env.traders[0].account.address()).unwrap();

    let adaptor_balance_before = query_balance_cw20(&env.app, &cw20_addr, adaptor_addr.clone()).unwrap();
    assert!(adaptor_balance_before.balance.is_zero());

    send_cw20(&env.app, &env.traders[0].account, &cw20_addr, adaptor_addr.clone(), mint_amount).unwrap();

    let adaptor_balance_after = query_balance_cw20(&env.app, &cw20_addr, adaptor_addr.clone()).unwrap();
    assert_eq!(adaptor_balance_after.balance, mint_amount);

    let trader_balance = env.get_balance(env.traders[0].account.address(), format!("factory/{}/{}", adaptor_addr, cw20_addr));
    assert_eq!(trader_balance, mint_amount);
}

#[test]
fn test_fail_receive_not_registered() {
    let env = Setup::new();

    let wasm = Wasm::new(&env.app);

    let adaptor_code_id = store_code(&wasm, &env.owner, "cw20_adapter".to_string());
    let base_code_id = store_code_with_path(
        &wasm,
        &env.owner,
        "../../contracts/cw20-adapter/src/testing/test_artifacts/cw20_base.wasm".to_string(),
    );

    let adaptor_addr = instantiate(&env.app, &env.owner, adaptor_code_id, "cw20-adapter");
    let cw20_addr = instantiate_cw20(&env.app, &env.owner, base_code_id, "cw20-base", "cw20-mofo");

    let mint_amount = Uint128::from(2_000_000u128);
    mint_cw20(&env.app, &env.owner, &cw20_addr, mint_amount, env.traders[0].account.address()).unwrap();

    let adaptor_balance_before = query_balance_cw20(&env.app, &cw20_addr, adaptor_addr.clone()).unwrap();
    assert!(adaptor_balance_before.balance.is_zero());

    let err = send_cw20(&env.app, &env.traders[0].account, &cw20_addr, adaptor_addr.clone(), mint_amount).unwrap_err();

    assert_eq!(
        err.to_string(),
        assert_execute_error("dispatch: submessages: Adapter is missing balance to create a new token-factory denom")
    );
}
