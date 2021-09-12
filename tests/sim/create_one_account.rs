use crate::utils::{get_public_keys, init_near_campaign, init_external_linkdrop};
use near_crypto::{InMemorySigner, SecretKey, Signer};
use near_sdk_sim::{call, to_yocto};
use std::str::FromStr;

const SK: &str =
  "39qnXSsiUUtuyMMJBkepa3qfv44qe6ZfixEMC9no1v6kjnaaKYj1pZ8pFmci1rSE9c2GsMVhF2NpXgu5aAYbCq3Y";

#[test]
fn create_one_account() {
  let (root, mut near_campaign) = init_near_campaign("5");
  init_external_linkdrop(&root);

  let public_keys = get_public_keys(0, 0);
  let new_public_key = get_public_keys(1, 1)[0].clone();

  call!(
    near_campaign.user_account,
    near_campaign.add_keys(public_keys)
  );

  // We want to sing transaction by new key;
  let claim_signer = InMemorySigner::from_secret_key(
    near_campaign.account_id().into(),
    SecretKey::from_str(SK).unwrap(),
  );
  near_campaign.user_account.signer = claim_signer.clone();

  // Create a new account
  call!(
    near_campaign.user_account,
    near_campaign.create_account_and_claim("john.testnet".parse().unwrap(), new_public_key)
  );

  {
    let runtime = root.borrow_runtime();

    // The new account should exist with 5 NEAR on the balance
    let john = runtime.view_account("john.testnet");
    assert_eq!(to_yocto("5"), john.unwrap().amount);

    // Used key should not exist after the successful 'claim'
    let key = runtime.view_access_key(
      near_campaign.account_id().as_str(),
      &claim_signer.public_key(),
    );
    assert_eq!(key.is_none(), true);
  }
}
