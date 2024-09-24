use crate::testing::utils::{instantiate, query_new_denom_fee, query_register_contract};

use cosmwasm_std::Coin;
use injective_test_tube::{Module, Wasm};
use injective_testing::test_tube::utils::store_code;
use testenv::utils::Setup;

#[test]
fn test_queries() {
    let env = Setup::new();

    let wasm = Wasm::new(&env.app);

    let code_id = store_code(&wasm, &env.owner, "cw20_adapter".to_string());

    let address = instantiate(&env.app, &env.owner, code_id, "cw20-adapter");

    let res = query_register_contract(&env.app, &address).unwrap();
    assert!(res.is_empty());

    let res = query_new_denom_fee(&env.app, &address).unwrap();
    assert_eq!(
        res,
        vec![Coin {
            amount: 10_000_000_000_000_000_000u128.into(),
            denom: "inj".to_string()
        }]
    );
}
