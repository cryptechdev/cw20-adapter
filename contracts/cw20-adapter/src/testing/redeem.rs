use crate::testing::utils::{instantiate, instantiate_cw20, mint_cw20, redeem_and_transfer, register_contract, send_cw20, store_code_with_path};

use cosmwasm_std::{coins, Uint128};
use injective_test_tube::{Account, Module, Wasm};
use injective_testing::test_tube::utils::store_code;
use testenv::utils::Setup;

use super::utils::query_balance_cw20;

#[test]
fn test_redeem() {
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

    send_cw20(&env.app, &env.traders[0].account, &cw20_addr, adaptor_addr.clone(), mint_amount).unwrap();

    let funds = coins(mint_amount.into(), format!("factory/{}/{}", adaptor_addr, cw20_addr.clone()));
    redeem_and_transfer(&env.app, &env.traders[0].account, &adaptor_addr, None, funds).unwrap();

    let trader_balance = env.get_balance(
        env.traders[0].account.address(),
        format!("factory/{}/{}", adaptor_addr, cw20_addr.clone()),
    );
    assert!(trader_balance.is_zero());

    let recipient_balance = query_balance_cw20(&env.app, &cw20_addr.clone(), env.traders[0].account.address()).unwrap();
    assert_eq!(recipient_balance.balance, mint_amount);
}

#[test]
fn test_redeem_and_transfer() {
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

    send_cw20(&env.app, &env.traders[0].account, &cw20_addr, adaptor_addr.clone(), mint_amount).unwrap();

    let funds = coins(mint_amount.into(), format!("factory/{}/{}", adaptor_addr, cw20_addr.clone()));
    redeem_and_transfer(
        &env.app,
        &env.traders[0].account,
        &adaptor_addr,
        Some(env.traders[6].account.address()),
        funds,
    )
    .unwrap();

    let trader_balance = env.get_balance(
        env.traders[0].account.address(),
        format!("factory/{}/{}", adaptor_addr, cw20_addr.clone()),
    );
    assert!(trader_balance.is_zero());

    let recipient_balance = query_balance_cw20(&env.app, &cw20_addr.clone(), env.traders[6].account.address()).unwrap();
    assert_eq!(recipient_balance.balance, mint_amount);
}
