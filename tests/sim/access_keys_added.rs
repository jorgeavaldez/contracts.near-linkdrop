use crate::utils::{get_public_keys, get_public_key_strings, init, init_external_linkdrop};
use near_crypto::{PublicKey};
use near_sdk_sim::{call};
use std::str::FromStr;

#[test]
fn access_keys_added() {
  let (root, near_campaign) = init("5");
  init_external_linkdrop(&root);

  let public_keys = get_public_keys(0, 0);
  let public_key_strings = get_public_key_strings(0, 0);
  let pk = PublicKey::from_str(public_key_strings[0]).unwrap();

  call!(
    near_campaign.user_account,
    near_campaign.add_keys(public_keys)
  );

  {
    let runtime = root.borrow_runtime();
    let key = runtime.view_access_key(near_campaign.account_id().as_str(), &pk);
    assert_eq!(key.is_some(), true);
  }
}
