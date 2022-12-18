use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId};

mod token_receiver;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub owner_id: AccountId,
    pub locked_nfts: UnorderedMap<String, AccountId>,
}

#[near_bindgen]
impl Contract {
    pub fn new(owner_id: AccountId) -> Self {
        Contract {
            owner_id,
            locked_nfts: UnorderedMap::new(b"m"),
        }
    }

    pub fn deposit(&mut self, _asset: AccountId, _amount: U128) {}

    pub fn supply_collateral_and_borrow(&mut self, position_id: u128) {
        let account_id = env::predecessor_account_id();
        self.assert_account_owns_nft_on_lending(position_id.to_string(), account_id);
    }

    pub fn return_collateral_and_repay(&mut self, collateral_id: u128) {
        let account_id = env::predecessor_account_id();
        self.assert_account_owns_nft_on_lending(collateral_id.to_string(), account_id);
    }

    pub fn liquidate(&mut self, _collateral_id: u128) {}

    fn assert_account_owns_nft_on_lending(&self, token_id: String, account_id: AccountId) {
        if let Some(owner) = self.locked_nfts.get(&token_id) {
            assert_eq!(
                owner, account_id,
                "User did not locked this NFT on lending contract"
            );
        } else {
            panic!("User did not locked this NFT on lending contract");
        }
    }
}
