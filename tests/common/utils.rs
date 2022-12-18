use mycelium_lab_lending::Contract;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::MockedBlockchain;
use near_sdk::{
    json_types::{ValidAccountId, U128},
    test_utils::{accounts, VMContextBuilder},
    testing_env,
};
use near_sdk_sim::to_yocto;

pub fn setup_contract() -> (VMContextBuilder, Contract) {
    let mut context = VMContextBuilder::new();
    testing_env!(context.predecessor_account_id(accounts(0)).build());
    let contract = Contract::new(accounts(0).to_string());
    (context, contract)
}

pub fn _deposit_tokens(
    context: &mut VMContextBuilder,
    contract: &mut Contract,
    account_id: ValidAccountId,
    token_id: ValidAccountId,
    amount: U128,
) {
    testing_env!(context
        .predecessor_account_id(token_id)
        .attached_deposit(to_yocto("1"))
        .build());
    contract.ft_on_transfer(account_id.clone(), amount, "".to_string());
}

#[allow(dead_code)]
pub fn withdraw_tokens(
    context: &mut VMContextBuilder,
    _contract: &mut Contract,
    account_id: ValidAccountId,
    _token_id: ValidAccountId,
    _amount: U128,
) {
    testing_env!(context
        .predecessor_account_id(account_id)
        .attached_deposit(to_yocto("1"))
        .build());
    // contract.withdraw(token_id.into(), amount.into());
}
