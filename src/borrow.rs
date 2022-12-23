use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::AccountId;

use crate::deposit::{BASIS_POINT_BASE, MS_IN_YEAR};

pub type BorrowId = u128;
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Borrow {
    pub asset: AccountId,
    pub amount: u128,
    pub collateral: u128,
    pub position_id: u128,
    pub health_factor: f64,
    pub last_update_timestamp: u64,
    pub apr: u16,
    pub fees: u128,
}

impl Borrow {
    pub fn update_timestamp(&mut self, current_timestamp: u64) {
        self.last_update_timestamp = current_timestamp;
    }

    pub fn calculate_fees(&self, current_timestamp: u64) -> u128 {
        let fees = (self.amount as f64)
            * Self::timestamp_difference_to_coefficient(
                current_timestamp - self.last_update_timestamp,
                self.apr,
            );
        fees.round() as u128
    }

    fn timestamp_difference_to_coefficient(timestamp_difference: u64, apr: u16) -> f64 {
        (timestamp_difference as f64 / MS_IN_YEAR as f64)
            * (1_f64 + apr as f64 / BASIS_POINT_BASE as f64)
    }

    pub fn refresh_fees(&mut self, current_timestamp: u64) {
        self.fees += self.calculate_fees(current_timestamp);
    }

    pub fn refresh_health_factor(&mut self) {
        self.health_factor = self.collateral as f64 / (self.amount as f64 + self.fees as f64);
    }

    pub fn assert_health_factor_is_above_1(&self) {
        assert!(self.health_factor >= 1.0);
    }

    pub fn assert_health_factor_is_under_1(&self) {
        assert!(self.health_factor < 1.0);
    }
}
