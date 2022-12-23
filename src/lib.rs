use std::collections::HashMap;

use balance::BalancesMap;
use borrow::{Borrow, BorrowId};
use deposit::{Deposit, DepositId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId};
use reserve::Reserve;

mod balance;
mod borrow;
mod deposit;
mod errors;
mod reserve;
mod token_receiver;

pub const APR_DEPOSIT: u16 = 500;
pub const APR_BORROW: u16 = 1000;
pub const BORROW_RATIO: f64 = 0.8;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub owner_id: AccountId,
    pub locked_nfts: UnorderedMap<String, AccountId>,
    pub balances_map: BalancesMap,
    pub deposits: HashMap<DepositId, Deposit>,
    pub deposits_created_number: DepositId,
    pub reserves: UnorderedMap<AccountId, Reserve>,
    pub borrows: UnorderedMap<BorrowId, Borrow>,
    pub borrows_number: BorrowId,
}

#[near_bindgen]
impl Contract {
    pub fn new(owner_id: AccountId) -> Self {
        Contract {
            owner_id,
            locked_nfts: UnorderedMap::new(b"m"),
            balances_map: UnorderedMap::new(b"b"),
            deposits: HashMap::new(),
            deposits_created_number: 0,
            reserves: UnorderedMap::new(b"r"),
            borrows: UnorderedMap::new(b"b"),
            borrows_number: 0,
        }
    }

    #[private]
    pub fn create_reserve(&mut self, reserve_token: AccountId) {
        let reserve = Reserve::default();
        self.reserves.insert(&reserve_token, &reserve);
    }

    pub fn create_deposit(&mut self, asset: AccountId, amount: U128) {
        let account_id = env::predecessor_account_id();
        let timestamp = env::block_timestamp();
        let deposit = Deposit {
            owner_id: account_id.clone(),
            asset: asset.clone(),
            amount: amount.0,
            timestamp,
            last_update_timestamp: timestamp,
            apr: APR_DEPOSIT,
            growth: 0,
        };
        self.deposits.insert(self.deposits_created_number, deposit);
        self.deposits_created_number += 1;
        self.decrease_balance(&account_id, &asset.to_string(), amount.0);
        let mut reserve = self.reserves.get(&asset).unwrap();
        reserve.increase_deposit(amount.0);
        self.reserves.insert(&asset, &reserve);
    }

    pub fn close_deposit(&mut self, deposit_id: U128) {
        let account_id = env::predecessor_account_id();
        if let Some(deposit) = self.deposits.remove(&deposit_id.0) {
            assert_eq!(deposit.owner_id, account_id, "You do not own this deposit");
            self.increase_balance(&account_id, &deposit.asset, deposit.amount);
            self.increase_balance(&account_id, &deposit.asset, deposit.growth);
            let mut reserve = self.reserves.get(&deposit.asset).unwrap();
            reserve.decrease_deposit(deposit.amount);
            self.reserves.insert(&deposit.asset, &reserve);
        } else {
            panic!("Deposit not found");
        }
    }

    pub fn refresh_deposits_growth(&mut self) {
        let current_timestamp = env::block_timestamp();
        for (_, deposit) in &mut self.deposits {
            deposit.refresh_growth(current_timestamp);
        }
    }

    #[allow(unused_assignments)]
    pub fn take_deposit_growth(&mut self, deposit_id: U128, amount: U128) -> U128 {
        let account_id = env::predecessor_account_id();
        let mut asset: Option<AccountId> = None;
        let mut growth = 0;
        if let Some(deposit) = self.deposits.get_mut(&deposit_id.0) {
            assert_eq!(deposit.owner_id, account_id, "You do not own this deposit");
            deposit.refresh_growth(env::block_timestamp());
            growth = deposit.take_growth(amount.0);
            asset = Some(deposit.asset.clone());
        } else {
            panic!("Deposit not found");
        }
        if let Some(asset) = asset {
            self.increase_balance(&account_id, &asset, growth);
            return growth.into();
        }
        0.into()
    }

    pub fn get_account_deposits(&self, account_id: AccountId) -> HashMap<DepositId, Deposit> {
        let mut result: HashMap<DepositId, Deposit> = HashMap::new();
        for (id, deposit) in &self.deposits {
            if deposit.owner_id == account_id {
                result.insert(*id, deposit.clone());
            }
        }
        result
    }

    pub fn get_total_locked_value_in_position(&self, _position_id: u128) {}

    pub fn supply_collateral_and_borrow(&mut self, position_id: u128) {
        let account_id = env::predecessor_account_id();
        self.assert_account_owns_nft_on_lending(position_id.to_string(), &account_id);
        // 0. somehow get (amount = total_value locked, token)on position
        let collateral = 0;
        let to_borrow = (BORROW_RATIO * collateral as f64).round() as u128; // health factor 1.25
        let token = String::new();
        self.increase_balance(&account_id, &token, to_borrow);
        let mut reserve = self.reserves.get(&token).unwrap();
        reserve.borrowed += to_borrow;
        self.reserves.insert(&token, &reserve);
        let borrow = Borrow {
            asset: token,
            amount: to_borrow,
            collateral,
            position_id,
            health_factor: 1.25,
            last_update_timestamp: env::block_timestamp(),
            apr: APR_BORROW,
            fees: 0,
        };
        self.borrows.insert(&self.borrows_number, &borrow);
        self.borrows_number += 1;
    }

    pub fn return_collateral_and_repay(&mut self, borrow_id: u128) {
        let account_id = env::predecessor_account_id();
        self.assert_account_owns_nft_on_lending(borrow_id.to_string(), &account_id);
        let borrow = self.borrows.remove(&borrow_id).unwrap();
        borrow.assert_health_factor_is_above_1();
        self.increase_balance(&account_id, &borrow.asset, borrow.amount - borrow.fees);
        let mut reserve = self.reserves.get(&borrow.asset).unwrap();
        reserve.borrowed -= borrow.amount;
        self.reserves.insert(&borrow.asset, &reserve);
        // send nft back ?
    }

    pub fn get_liquidation_list(&self) -> Vec<BorrowId> {
        let mut result = Vec::new();
        for (id, borrow) in self.borrows.iter() {
            if borrow.health_factor < 1.0 {
                result.push(id);
            }
        }
        result
    }

    pub fn liquidate(&mut self, borrow_id: BorrowId, amount_in: U128, amount_out: U128) {
        if let Some(mut borrow) = self.borrows.get(&borrow_id) {
            borrow.assert_health_factor_is_under_1();
            let discount = (1.0 - borrow.health_factor) / 2.0;
            let discounted_collateral_sum = amount_out.0 * (1 - discount as u128);
            assert!(discounted_collateral_sum <= amount_in.0);
            borrow.collateral -= amount_out.0;
            borrow.amount -= amount_out.0;
            borrow.refresh_health_factor();
            borrow.assert_health_factor_is_under_1();
            self.borrows.insert(&borrow_id, &borrow);
        } else {
            panic!("Borrow not found");
        }
    }

    fn assert_account_owns_nft_on_lending(&self, token_id: String, account_id: &AccountId) {
        if let Some(owner) = self.locked_nfts.get(&token_id) {
            assert_eq!(
                &owner, account_id,
                "User did not locked this NFT on lending contract"
            );
        } else {
            panic!("User did not locked this NFT on lending contract");
        }
    }
}
