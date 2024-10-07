use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, Coin, Uint128};
use cw20::{BalanceResponse, Cw20Coin, MinterResponse};
use cw20_base::msg::{ExecuteMsg as Cw20ExecuteMsg, InstantiateMarketingInfo, QueryMsg as Cw20QueryMsg};
use injective_test_tube::{
    injective_std::types::cosmwasm::wasm::v1::MsgExecuteContractResponse, Account, InjectiveTestApp, Module, RunnerExecuteResult, RunnerResult,
    SigningAccount, Wasm,
};

#[cw_serde]
pub struct Cw20InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_balances: Vec<Cw20Coin>,
    pub mint: Option<MinterResponse>,
    pub marketing: Option<InstantiateMarketingInfo>,
}

pub fn assert_execute_error(message: &str) -> String {
    format!(
        "execute error: failed to execute message; message index: 0: {}: execute wasm contract failed",
        message
    )
}

pub fn store_code_with_path(wasm: &Wasm<InjectiveTestApp>, owner: &SigningAccount, path: String) -> u64 {
    let wasm_byte_code = std::fs::read(path).unwrap();
    wasm.store_code(&wasm_byte_code, None, owner).unwrap().data.code_id
}

pub fn instantiate(app: &InjectiveTestApp, owner: &SigningAccount, code_id: u64, label: &str) -> String {
    let wasm = Wasm::new(app);

    wasm.instantiate(code_id, &InstantiateMsg {}, Some(&owner.address()), Some(label), &[], owner)
        .unwrap()
        .data
        .address
}

pub fn instantiate_cw20(app: &InjectiveTestApp, owner: &SigningAccount, code_id: u64, label: &str, name: &str) -> String {
    let wasm = Wasm::new(app);

    wasm.instantiate(
        code_id,
        &Cw20InstantiateMsg {
            name: name.to_string(),
            symbol: "ERC".to_string(),
            decimals: 6,
            initial_balances: vec![],
            mint: Some(MinterResponse {
                minter: owner.address().to_string(),
                cap: None,
            }),
            marketing: None,
        },
        Some(&owner.address()),
        Some(label),
        &[],
        owner,
    )
    .unwrap()
    .data
    .address
}

pub fn mint_cw20(
    app: &InjectiveTestApp,
    owner: &SigningAccount,
    contract: &str,
    amount: Uint128,
    recipient: String,
) -> RunnerExecuteResult<MsgExecuteContractResponse> {
    let wasm = Wasm::new(app);

    wasm.execute(contract, &Cw20ExecuteMsg::Mint { recipient, amount }, &[], owner)
}

pub fn send_cw20(
    app: &InjectiveTestApp,
    owner: &SigningAccount,
    contract: &str,
    recipient: String,
    amount: Uint128,
) -> RunnerExecuteResult<MsgExecuteContractResponse> {
    let wasm = Wasm::new(app);

    wasm.execute(
        contract,
        &Cw20ExecuteMsg::Send {
            contract: recipient,
            amount,
            msg: Binary::default(),
        },
        &[],
        owner,
    )
}

pub fn register_contract(
    app: &InjectiveTestApp,
    owner: &SigningAccount,
    contract: &str,
    addr: &str,
    funds: Vec<Coin>,
) -> RunnerExecuteResult<MsgExecuteContractResponse> {
    let wasm = Wasm::new(app);

    wasm.execute(contract, &ExecuteMsg::RegisterCw20Contract { addr: Addr::unchecked(addr) }, &funds, owner)
}

pub fn redeem_and_transfer(
    app: &InjectiveTestApp,
    owner: &SigningAccount,
    contract: &str,
    recipient: Option<String>,
    funds: Vec<Coin>,
) -> RunnerExecuteResult<MsgExecuteContractResponse> {
    let wasm = Wasm::new(app);

    wasm.execute(contract, &ExecuteMsg::RedeemAndTransfer { recipient }, &funds, owner)
}

pub fn query_register_contract(app: &InjectiveTestApp, contract: &str) -> RunnerResult<Vec<String>> {
    let wasm = Wasm::new(app);

    wasm.query(contract, &QueryMsg::RegisteredContracts {})
}

pub fn query_new_denom_fee(app: &InjectiveTestApp, contract: &str) -> RunnerResult<Vec<Coin>> {
    let wasm = Wasm::new(app);

    wasm.query(contract, &QueryMsg::NewDenomFee {})
}

pub fn query_balance_cw20(app: &InjectiveTestApp, contract: &str, address: String) -> RunnerResult<BalanceResponse> {
    let wasm = Wasm::new(app);

    wasm.query(contract, &Cw20QueryMsg::Balance { address })
}
