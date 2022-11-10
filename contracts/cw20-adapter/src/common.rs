use cosmwasm_std::Addr;
use regex::{Captures, Regex};

// TODO: replace with extension
// TODO: optimise so it's compiled once. Or replace with ordinary split

// static TOKEN_FACTORY_EXPR: Regex = Regex::new("factory/([A-Za-z0-9]{44})/([A-Za-z0-9]{44})").unwrap();

fn parser() -> Regex { Regex::new("factory/([A-Za-z0-9]{44})/([A-Za-z0-9]{44})").unwrap() }

pub fn is_token_factory_denom(denom: &str) -> bool {
    parser().is_match(denom)
}

pub fn get_cw20_address_from_denom(denom: &str) -> Option<&str> {
    let captures = parser().captures(denom)?;
    let cw20addr = captures.get(2)?;
    Some(cw20addr.as_str())
}

pub fn get_denom(master_address: &Addr, vault_address: &Addr) -> String {
    format!("factory/{}/{}", master_address, vault_address)
}
