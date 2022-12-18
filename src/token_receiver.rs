use crate::*;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_contract_standards::non_fungible_token::core::NonFungibleTokenReceiver;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::json_types::ValidAccountId;
use near_sdk::{json_types::U128, near_bindgen, PromiseOrValue};

#[near_bindgen]
impl NonFungibleTokenReceiver for Contract {
    #[allow(unreachable_code)]
    #[allow(unused_variables)]
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> PromiseOrValue<bool> {
        self.locked_nfts.insert(&token_id, &sender_id);
        PromiseOrValue::Value(true)
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    #[allow(unreachable_code)]
    #[allow(unused_variables)]
    fn ft_on_transfer(
        &mut self,
        sender_id: ValidAccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        PromiseOrValue::Value(U128(0))
    }
}
