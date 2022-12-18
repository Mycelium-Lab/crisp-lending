use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    AccountId,
};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Deposit {
    pub owner_id: AccountId,
    pub asset: AccountId,
    pub amount: u128,
    pub timestamp: u64,
}
