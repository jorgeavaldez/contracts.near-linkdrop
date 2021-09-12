use crate::utils::{init_near_campaign_with_timestamp, init_external_linkdrop};
use near_sdk_sim::{call, to_ts, ExecutionResult};

#[test]
fn set_campaign_dates() {
  let (root, near_campaign) = init_near_campaign_with_timestamp("5", 1, to_ts(3));
  init_external_linkdrop(&root);

  let valid_call_result: ExecutionResult = call!(
    near_campaign.user_account,
    near_campaign.set_start_and_end_date(1, to_ts(4))
  );

  println!("\nSet campaign dates valid: {:#?}\n", valid_call_result);
  assert!(valid_call_result.is_ok());

  let invalid_call_result: ExecutionResult = call!(
    near_campaign.user_account,
    near_campaign.set_start_and_end_date(to_ts(2), 1)
  );

  println!("\nSet campaign dates invalid: {:#?}\n", invalid_call_result);
  assert!(!invalid_call_result.is_ok());
}
