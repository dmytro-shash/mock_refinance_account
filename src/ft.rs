use crate::*;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::json_types::U128;
use near_sdk::AccountId;
use near_sdk::{log, serde_json, Balance, PromiseOrValue};
use crate::utils::Action;

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        assert!(
            Balance::from(amount) > 0,
            "Amount should be a positive number"
        );

        log!(format!("sender_id {}, msg {}", sender_id, msg));

        let action: Action = serde_json::from_str(&msg).expect("Incorrect command in transfer");

        match action {
            Action::Swap(swap_action) => {
                swap_action;
                PromiseOrValue::Value(U128(0))
            }
            _ => {
                panic!("Incorrect action in transfer")
            }
        }
    }
}
