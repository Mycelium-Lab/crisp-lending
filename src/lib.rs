use balance::BalancesMap;
use borrow::Borrow;
use deposit::Deposit;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId};

mod balance;
mod borrow;
mod deposit;
mod errors;
mod token_receiver;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub owner_id: AccountId,
    pub locked_nfts: UnorderedMap<String, AccountId>,
    pub balances_map: BalancesMap,
    pub deposits: UnorderedMap<u128, Deposit>,
    pub deposits_created_number: u128,
    pub deposits_per_owner: UnorderedMap<u128, Deposit>,
    pub reserves: UnorderedMap<AccountId, u128>,
    pub borrows: UnorderedMap<u128, Borrow>,
    pub borrows_number: u128,
    pub borrows_per_owner: UnorderedMap<u128, Borrow>,
}

#[near_bindgen]
impl Contract {
    pub fn new(owner_id: AccountId) -> Self {
        Contract {
            owner_id,
            locked_nfts: UnorderedMap::new(b"m"),
            balances_map: UnorderedMap::new(b"b"),
            deposits: UnorderedMap::new(b"d"),
            deposits_created_number: 0,
            deposits_per_owner: UnorderedMap::new(b"o"),
            reserves: UnorderedMap::new(b"r"),
            borrows: UnorderedMap::new(b"b"),
            borrows_number: 0,
            borrows_per_owner: UnorderedMap::new(b"w"),
        }
    }

    pub fn create_deposit(&mut self, asset: AccountId, amount: U128) {
        let account_id = env::predecessor_account_id();
        let timestamp = env::block_timestamp();
        let deposit = Deposit {
            owner_id: account_id.clone(),
            asset: asset.clone(),
            amount: amount.0,
            timestamp,
        };
        self.deposits
            .insert(&self.deposits_created_number, &deposit);
        self.deposits_per_owner
            .insert(&self.deposits_created_number, &deposit);
        self.deposits_created_number += 1;
        self.decrease_balance(&account_id, &asset.to_string(), amount.0);
        let reserves_amount = self.reserves.get(&asset.to_string()).unwrap_or(0);
        self.reserves
            .insert(&asset.to_string(), &(reserves_amount + amount.0));
        // TO DO
    }

    pub fn close_deposit(&mut self, deposit_id: u128) {
        let account_id = env::predecessor_account_id();
        if let Some(deposit) = self.deposits.remove(&deposit_id) {
            self.increase_balance(&account_id, &deposit.asset, deposit.amount);
            self.deposits.remove(&deposit_id);
        }
        // TO DO
    }

    pub fn supply_collateral_and_borrow(&mut self, position_id: u128) {
        let account_id = env::predecessor_account_id();
        self.assert_account_owns_nft_on_lending(position_id.to_string(), account_id);
        // TO DO
    }

    pub fn return_collateral_and_repay(&mut self, borrow_id: u128) {
        let account_id = env::predecessor_account_id();
        self.assert_account_owns_nft_on_lending(borrow_id.to_string(), account_id);
        // TO DO
    }

    pub fn liquidate(&mut self, _borrow_id: u128) {
        // TO DO
    }

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
