use cosmwasm_std::{Addr, Uint128};
use injective_cosmwasm::{checked_address_to_subaccount_id, SubaccountId};
use injective_std::types::cosmos::bank::v1beta1::QueryBalanceRequest;
use injective_test_tube::{Account, Bank, InjectiveTestApp, Module, SigningAccount};
use injective_testing::{
    mocks::{MOCK_ATOM_DECIMALS, MOCK_ATOM_DENOM, MOCK_BASE_DECIMALS, MOCK_BASE_DENOM, MOCK_QUOTE_DECIMALS, MOCK_QUOTE_DENOM},
    utils::str_coin,
};
use std::{collections::HashMap, str::FromStr};

pub const MOCK_MARKET_ID: &str = "0x427aee334987c52fa7b567b2662bdbb68614e48c000000000000000000000000";
pub const MOCK_TIA_DENOM: &str = "tia";

pub struct TraderInfo {
    pub account: SigningAccount,
    pub subaccount_id: SubaccountId,
}

pub struct Setup {
    pub app: InjectiveTestApp,
    pub owner: SigningAccount,
    pub traders: Vec<TraderInfo>,
    pub denoms: HashMap<String, String>,
}

impl Setup {
    pub fn new() -> Self {
        let app = InjectiveTestApp::new();

        let mut denoms = HashMap::new();
        denoms.insert("atom".to_string(), MOCK_ATOM_DENOM.to_string());
        denoms.insert("quote".to_string(), MOCK_QUOTE_DENOM.to_string());
        denoms.insert("base".to_string(), MOCK_BASE_DENOM.to_string());
        denoms.insert("tia".to_string(), MOCK_TIA_DENOM.to_string());

        let owner = app
            .init_account(&[
                str_coin("1000000", MOCK_ATOM_DENOM, MOCK_ATOM_DECIMALS),
                str_coin("1000000", MOCK_BASE_DENOM, MOCK_BASE_DECIMALS),
                str_coin("1000000", MOCK_QUOTE_DENOM, MOCK_QUOTE_DECIMALS),
                str_coin("1000000", MOCK_TIA_DENOM, MOCK_QUOTE_DECIMALS),
            ])
            .unwrap();

        let mut traders: Vec<TraderInfo> = Vec::new();
        for _ in 0..10 {
            let trader = app.init_account(&[str_coin("1000000", MOCK_BASE_DENOM, MOCK_BASE_DECIMALS)]).unwrap();

            let trader_subaccount_id = checked_address_to_subaccount_id(&Addr::unchecked(trader.address()), 1u32);

            traders.push(TraderInfo {
                account: trader,
                subaccount_id: trader_subaccount_id,
            });
        }

        Self { app, owner, traders, denoms }
    }

    pub fn get_balance(&self, address: String, denom: String) -> Uint128 {
        let bank = Bank::new(&self.app);

        let response = bank.query_balance(&QueryBalanceRequest { address, denom }).unwrap();

        match response.balance {
            Some(balance) => Uint128::from_str(&balance.amount).unwrap(),
            None => Uint128::zero(),
        }
    }
}

impl Default for Setup {
    fn default() -> Self {
        Self::new()
    }
}
