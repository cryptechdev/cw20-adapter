use crate::{
    error::ContractError,
    testing::utils::{assert_execute_error, instantiate, instantiate_cw20, query_register_contract, register_contract, store_code_with_path},
};

use cosmwasm_std::coins;
use injective_test_tube::{Module, Wasm};
use injective_testing::{mocks::MOCK_ATOM_DENOM, test_tube::utils::store_code};
use testenv::utils::Setup;

#[test]
fn test_register() {
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

    let res = query_register_contract(&env.app, &adaptor_addr).unwrap();
    assert_eq!(res, vec![cw20_addr.to_string()]);
}

#[test]
fn test_register_additional_funds() {
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

    let deposit = coins(10_000_000_000_000_000_001u128, "inj");
    let err = register_contract(&env.app, &env.owner, &adaptor_addr, &cw20_addr, deposit).unwrap_err();

    assert_eq!(
        err.to_string(),
        assert_execute_error(&ContractError::SuperfluousFundsProvided.to_string())
    );
}

#[test]
fn test_register_twice() {
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
    register_contract(&env.app, &env.owner, &adaptor_addr, &cw20_addr, deposit.clone()).unwrap();

    let err = register_contract(&env.app, &env.owner, &adaptor_addr, &cw20_addr, deposit).unwrap_err();

    assert_eq!(
        err.to_string(),
        assert_execute_error("CW-20 contract with the same address was already registered")
    );
}

#[test]
fn test_register_insufficient_funds() {
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

    let deposit = coins(9_999_999_999_999_999_999u128, "inj");
    let err = register_contract(&env.app, &env.owner, &adaptor_addr, &cw20_addr, deposit).unwrap_err();

    assert_eq!(
        err.to_string(),
        assert_execute_error("Adapter is missing balance to create a new token-factory denom")
    );
}

#[test]
fn test_register_invalid_funds() {
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

    let deposit = coins(1_000_000u128, MOCK_ATOM_DENOM);
    let err = register_contract(&env.app, &env.owner, &adaptor_addr, &cw20_addr, deposit).unwrap_err();

    assert_eq!(
        err.to_string(),
        assert_execute_error("Adapter is missing balance to create a new token-factory denom")
    );
}
