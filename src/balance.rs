use near_contract_standards::fungible_token::core_impl::ext_fungible_token;
use near_sdk::env;
use near_sdk::{collections::UnorderedMap, json_types::U128, AccountId};

use crate::errors::{BALANCE_NOT_FOUND, TOKEN_HAS_NOT_BEEN_DEPOSITED};
use crate::Contract;

pub type BalancesMap = UnorderedMap<AccountId, Balance>;
type Balance = UnorderedMap<AccountId, u128>;

pub const GAS_FOR_FT_TRANSFER: u64 = 20_000_000_000_000;

impl Contract {
    fn assert_owner() {
        assert_eq!(env::current_account_id(), env::predecessor_account_id());
    }

    pub fn deposit_ft(&mut self, account_id: &AccountId, token_in: &AccountId, amount: u128) {
        Contract::assert_owner();
        if let Some(mut balance) = self.balances_map.get(account_id) {
            let current_value = balance.get(token_in).unwrap_or(0);
            let new_value = current_value + amount;
            balance.insert(token_in, &new_value);
            self.balances_map.insert(account_id, &balance);
        } else {
            let mut balance = UnorderedMap::new(account_id.clone().into_bytes());
            balance.insert(&token_in.to_string(), &amount);
            self.balances_map.insert(account_id, &balance);
        }
    }

    pub fn balance_withdraw(&mut self, account_id: &AccountId, token: &AccountId, amount: u128) {
        Contract::assert_owner();
        if let Some(mut balance) = self.balances_map.get(account_id) {
            if let Some(current_amount) = balance.get(token) {
                let message = format!(
                    "Not enough tokens. You want to withdraw {} of {} but only have {}",
                    amount, token, current_amount
                );
                assert!(amount <= current_amount, "{}", message);
                balance.insert(token, &(current_amount - amount));
                self.balances_map.insert(account_id, &balance);
                ext_fungible_token::ft_transfer(
                    account_id.to_string(),
                    U128(amount),
                    None,
                    &token,
                    1,
                    GAS_FOR_FT_TRANSFER,
                );
                return;
            }
        }
        panic!("{}", TOKEN_HAS_NOT_BEEN_DEPOSITED);
    }

    pub fn decrease_balance(&mut self, account_id: &AccountId, token: &AccountId, amount: u128) {
        Contract::assert_owner();
        if let Some(mut balance) = self.balances_map.get(account_id) {
            if let Some(current_amount) = balance.get(token) {
                let message = format!("Not enough tokens. You want to decrease your balance on {} of {} but only have {}", amount, token, current_amount);
                assert!(amount <= current_amount, "{}", message);
                balance.insert(token, &(current_amount - amount));
                self.balances_map.insert(account_id, &balance);
            }
        } else {
            let message = format!(
                "Not enough tokens. You want to decrease your balance on {} of {} but only have {}",
                amount, token, 0
            );
            panic!("{}", message);
        }
    }

    pub fn increase_balance(&mut self, account_id: &AccountId, token: &AccountId, amount: u128) {
        Contract::assert_owner();
        if let Some(mut balance) = self.balances_map.get(account_id) {
            if let Some(current_amount) = balance.get(token) {
                balance.insert(token, &(current_amount + amount));
                self.balances_map.insert(account_id, &balance);
            } else {
                balance.insert(token, &amount);
                self.balances_map.insert(account_id, &balance);
            }
        } else {
            panic!("{}", BALANCE_NOT_FOUND);
        }
    }
}
