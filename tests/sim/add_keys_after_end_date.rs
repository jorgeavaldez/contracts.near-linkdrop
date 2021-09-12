use crate::utils::{get_public_keys, init_near_campaign_with_timestamp, init_external_linkdrop};
use near_sdk_sim::{call, to_ts, ExecutionResult};

#[test]
fn add_keys_after_end_date() {
  let (root, near_campaign) = init_near_campaign_with_timestamp("5", 1, to_ts(2));
  init_external_linkdrop(&root);

  let public_keys = get_public_keys(0, 1);

  let result: ExecutionResult = call!(
    near_campaign.user_account,
    near_campaign.add_keys(public_keys)
  );

  assert_eq!(result.is_ok(), true);
  assert_eq!(result.unwrap_json_value(), 0);
}
