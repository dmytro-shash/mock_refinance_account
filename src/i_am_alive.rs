use std::arch::global_asm;
use crate::*;
use near_sdk::near_bindgen;


#[near_bindgen]
impl Contract {
    pub fn set_balance(&mut self, account: AccountId, amount: U128) {
        self.some_test_field.insert(&account, &amount.0);
    }

    pub fn view_balance(&self, account: AccountId) -> U128 {
        U128::from(self.some_test_field.get(&account).unwrap())
    }
}

