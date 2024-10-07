use crate::msg::InstantiateMsg;

use injective_test_tube::{Account, Module, Wasm};
use injective_testing::test_tube::utils::store_code;
use testenv::utils::Setup;

#[test]
fn test_instantiation() {
    let env = Setup::new();

    let wasm = Wasm::new(&env.app);

    let code_id = store_code(&wasm, &env.owner, "cw20_adapter".to_string());

    let res = wasm
        .instantiate(
            code_id,
            &InstantiateMsg {},
            Some(&env.owner.address()),
            Some("cw20-adaptor-contract"),
            &[],
            &env.owner,
        )
        .unwrap();

    let attributes = res
        .events
        .iter()
        .find(|e| e.ty == "cosmwasm.wasm.v1.EventContractInstantiated")
        .unwrap()
        .attributes
        .clone();

    let admin = attributes.iter().find(|e| e.key == "admin").unwrap();
    let creator = attributes.iter().find(|e| e.key == "creator").unwrap();
    let label = attributes.iter().find(|e| e.key == "label").unwrap();

    assert_eq!(admin.value, format!("\"{}\"", env.owner.address()));
    assert_eq!(creator.value, format!("\"{}\"", env.owner.address()));
    assert_eq!(label.value, format!("\"{}\"", "cw20-adaptor-contract"));
}
