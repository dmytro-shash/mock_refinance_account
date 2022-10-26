use std::ops::Add;
use crate::*;
use near_sdk::near_bindgen;
use near_sdk::serde::de::Unexpected::Str;
use crate::utils::{LptId, PoolId};


#[near_bindgen]
impl Contract {
    pub fn add_liquidity(
        &mut self,
        pool_id: PoolId,
        left_point: i32,
        right_point: i32,
        amount_x: U128,
        amount_y: U128,
        min_amount_x: U128,
        min_amount_y: U128,
    ) -> LptId {
        /// If user has a LPT with same pool_id&pl&pr,
        /// it is an increase operation, else mint.
        /// cause there is a UnorderedMap<pool_id:lp:rp, lptid>; per user.
        /// @param pool_id: a string like token_a|token_b|fee
        /// @param left_point: left point of this range
        /// @param right_point: right point of this range
        /// @param amount_x: the number of token X users expect to add liquidity to use
        /// @param amount_y: the number of token Y users expect to add liquidity to use
        /// @param min_amount_y: the minimum number of token Y users expect to add liquidity to use
        /// @param min_amount_x: the minimum number of token X users expect to add liquidity to use
        /// @return the exist or new-mint lp token id, a string like pool_id|inner_id

        let mut result = String::new();

        result.push(pool_id.parse().unwrap());
        result.push("|".parse().unwrap());
        result.add(stringify!(env::block_height()))
    }

    pub fn remove_liquidity(
        &mut self,
        lpt_id: LptId,
        amount: U128,
        min_amount_x: U128,
        min_amount_y: U128,
    ) -> (U128, U128) {
        /// If all amount in this lp token is removed,
        /// it will be a burn operation else decrease amount of this lp token
        /// @param lpt_id: a string like pool_id|inner_id
        /// @param amount: amount of liquidity.
        /// @param min_amount_x: removing liquidity will at least give you the number of token X
        /// @param min_amount_y: removing liquidity will at least give you the number of token Y
        /// @return (amount_x, amount_y)
        /// amount_x, balance of tokenX released into inner account (including feeX)
        /// amount_y, balance of tokenY released into inner account (including feeY);
        /// Note: remove_liquidity with 0 amount, 0 min_amount_x, 0 min_amount_y means claim


        return (U128::from(min_amount_x.0 + env::block_height() as u128), U128::from(min_amount_y.0 + env::block_height() as u128));
    }
}

