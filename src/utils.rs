use near_sdk::BorshStorageKey;
use crate::*;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::borsh::BorshSerialize;


pub type PoolId = String;
// param pool_id: a string like token_a|token_b|fee

pub type LptId = String;
// param lpt_id: a string like pool_id|inner_id


/// Single action. Allows to execute sequence of various actions initiated by an account.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum Action {
    Swap(SwapAction),
}


/// Single swap action.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SwapAction {
    /// Pool which should be used for swapping.
    pub pool_id: u64,
    /// Token to swap from.
    pub token_in: AccountId,
    /// Amount to exchange.
    /// If amount_in is None, it will take amount_out from previous step.
    /// Will fail if amount_in is None on the first step.
    pub amount_in: Option<U128>,
    /// Token to swap into.
    pub token_out: AccountId,
    /// Required minimum amount of token_out.
    pub min_amount_out: U128,
}


#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKeys {
    TestFieldStorageKey
}