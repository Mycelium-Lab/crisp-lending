use near_sdk::AccountId;

pub struct Borrow {
    pub asset: AccountId,
    pub amount: u128,
    pub heath_factor: u128,
}
