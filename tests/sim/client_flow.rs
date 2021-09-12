use crate::utils::{init_internal_linkdrop, init_external_linkdrop};
use near_sdk::serde_json::json;
use near_crypto::{InMemorySigner, SecretKey, KeyType};
use near_sdk::PublicKey;
use near_sdk::borsh::BorshSerialize;
use near_sdk_sim::{call, DEFAULT_GAS, UserAccount, to_yocto, STORAGE_AMOUNT, ExecutionResult};
use near_sdk_sim::transaction::{Transaction, SignedTransaction, ExecutionStatus, ExecutionOutcome};
use near_sdk_sim::hash::CryptoHash;
use std::convert::TryFrom;

#[test]
fn client_flow() {
  let (root, linkdrop) = init_internal_linkdrop();
  init_external_linkdrop(&root);

  let alice: UserAccount = root.create_user("alice".parse().unwrap(), to_yocto("200"));

  let alice_user_secret_key = SecretKey::from_random(KeyType::ED25519);
  let alice_user_signer = InMemorySigner::from_secret_key(String::from("alice.linkdrop"), alice_user_secret_key);
  let alice_user_pk: PublicKey = PublicKey::try_from(alice_user_signer.public_key.try_to_vec().unwrap()).unwrap();

  // Test can create a user account
  let create_user_result: ExecutionResult = call!(
    alice,
    linkdrop.create_user_account(alice_user_pk),
    deposit = STORAGE_AMOUNT + to_yocto("5")
  );

  assert!(create_user_result.is_ok());

  {
    let runtime = root.borrow_runtime();
    let user_contract_account = runtime.view_account("alice.linkdrop");
    assert!(user_contract_account.is_some());
    assert!(runtime.view_access_key("alice.linkdrop", &alice_user_signer.public_key).is_some());
  }

  {
    let runtime = root.borrow_runtime();
    let user_contract_account = runtime.view_account("foo.alice.linkdrop");
    println!("\nCampaign account does not exist? {:#?}\n", user_contract_account);
    assert!(user_contract_account.is_none());
    assert!(runtime.view_access_key("foo.alice.linkdrop", &alice_user_signer.public_key).is_none());
  }

  // create a campaign
  {
    let mut runtime = root.borrow_runtime_mut();
    let nonce = runtime.view_access_key("alice.linkdrop", &alice_user_signer.public_key).unwrap().nonce + 1;

    // this is the only way i was able to call the campaign create using the generated keys
    let tx: SignedTransaction = Transaction::new(
      String::from("alice.linkdrop"),
      alice_user_signer.public_key.clone(),
      String::from("alice.linkdrop"),
      nonce,
      CryptoHash::default()
    ).function_call(
      "create_near_campaign".to_string(),
      json!({
        "name": "foo",
        "public_key": alice_user_signer.public_key.clone(),
        "tokens_per_key": to_yocto("1").to_string(),
      }).to_string().into_bytes(),
      DEFAULT_GAS,
      STORAGE_AMOUNT,
    ).sign(&alice_user_signer);


    let (_hash, outcome) = runtime.resolve_tx(tx).unwrap();
    runtime.process_all().unwrap();

    println!("\nalice.linkdrop#create_near_campaign: {:#?}\n", outcome);

    assert!(matches!(
      outcome,
      ExecutionOutcome { status: ExecutionStatus::SuccessValue(_), .. },
    ));
  }

  {
    let runtime = root.borrow_runtime();
    let user_contract_account = runtime.view_account("foo.alice.linkdrop");
    println!("\nCampaign account exists? {:#?}\n", user_contract_account);
    assert!(user_contract_account.is_some());
    assert!(runtime.view_access_key("foo.alice.linkdrop", &alice_user_signer.public_key).is_some());
  }

  // Add keys
  /*
  {
    let mut runtime = root.borrow_runtime_mut();
    let nonce = runtime.view_access_key("foo.alice.linkdrop", &alice_user_signer.public_key).unwrap().nonce + 1;

    let tx: SignedTransaction = Transaction::new(
      String::from("foo.alice.linkdrop"),
      alice_user_signer.public_key.clone(),
      String::from("foo.alice.linkdrop"),
      nonce,
      CryptoHash::default()
    ).function_call(
      "add_keys".to_string(),
      json!({
        "keys": "foo", // TODO: add the vector of keys here
      }).to_string().into_bytes(),
      DEFAULT_GAS,
      STORAGE_AMOUNT,
    ).sign(&alice_user_signer);


    let (_hash, outcome) = runtime.resolve_tx(tx).unwrap();
    runtime.process_all().unwrap();

    println!("foo.alice.linkdrop#add_keys: {:#?}", outcome);

    assert!(matches!(
      outcome,
      ExecutionOutcome { status: ExecutionStatus::SuccessValue(_), .. },
    ));
  }
  */
}
