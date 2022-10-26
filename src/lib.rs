mod ft;
mod add_remove_liquidity;
mod utils;


use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, require, AccountId, Balance, PromiseOrValue};
use std::collections::HashMap;
use crate::utils::StorageKeys;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    some_test_field: UnorderedMap<AccountId, Balance>,
}

impl Default for Contract {
    fn default() -> Self {
        env::panic_str("Margin trading contract should be initialized before usage")
    }
}


#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    pub fn new() -> Self {
        require!(!env::state_exists(), "Already initialized");

        Self { some_test_field: UnorderedMap::new(StorageKeys::TestFieldStorageKey) }
    }
}
