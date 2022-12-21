use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    AccountId,
};

pub const MS_IN_YEAR: u64 = 31536000000;
pub const BASIS_POINT_BASE: u16 = 10000;

pub type DepositId = u128;

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct Deposit {
    pub owner_id: AccountId,
    pub asset: AccountId,
    pub amount: u128,
    pub timestamp: u64,
    pub last_update_timestamp: u64,
    pub apr: u16,
    pub growth: u128,
}

impl Deposit {
    pub fn update_timestamp(&mut self, current_timestamp: u64) {
        self.last_update_timestamp = current_timestamp;
    }

    pub fn calculate_growth(&self, current_timestamp: u64) -> u128 {
        let growth = (self.amount as f64)
            * Self::timestamp_difference_to_coefficient(
                current_timestamp - self.last_update_timestamp,
                self.apr,
            );
        growth.round() as u128
    }

    fn timestamp_difference_to_coefficient(timestamp_difference: u64, apr: u16) -> f64 {
        (timestamp_difference as f64 / MS_IN_YEAR as f64)
            * (1_f64 + apr as f64 / BASIS_POINT_BASE as f64)
    }

    pub fn refresh_growth(&mut self, current_timestamp: u64) {
        self.growth += self.calculate_growth(current_timestamp);
    }

    pub fn take_growth(&mut self, amount: u128) -> u128 {
        if amount > self.growth {
            let result = self.growth;
            self.growth = 0;
            result
        } else {
            self.growth -= amount;
            amount
        }
    }
}
