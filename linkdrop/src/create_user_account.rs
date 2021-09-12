use crate::*;
use near_sdk::{AccountId, PublicKey, log};

#[near_bindgen]
impl Linkdrop {
  #[payable]
  pub fn create_user_account(name: AccountId, public_key: PublicKey) -> Promise {
    let account_id = AccountId::new_unchecked(format!("{}.{}", name, env::current_account_id()));

    log!("Account ID: {:#?}", account_id.as_str());

    Promise::new(account_id)
      .create_account()
      .transfer(env::attached_deposit())
      .add_full_access_key(public_key.into())
      .deploy_contract(USER_WASM.to_vec())
  }
}
