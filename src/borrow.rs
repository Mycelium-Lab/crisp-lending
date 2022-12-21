use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::AccountId;

pub type BorrowId = u128;
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Borrow {
    pub asset: AccountId,
    pub amount: u128,
    pub health_factor: f64,
    pub last_update_timestamp: u64,
}

impl Borrow {
    pub fn update_timestamp(&mut self, current_timestamp: u64) {
        self.last_update_timestamp = current_timestamp;
    }
}
