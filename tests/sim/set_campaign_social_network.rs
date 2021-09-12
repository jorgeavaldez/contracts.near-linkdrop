use crate::utils::{init_near_campaign_with_timestamp, init_external_linkdrop};
use near_sdk_sim::{call, view, to_ts, ExecutionResult};
use near_campaign::get_campaign_metadata::Metadata;

#[test]
fn set_campaign_social_network() {
  let (root, near_campaign) = init_near_campaign_with_timestamp("5", 1, to_ts(3));
  init_external_linkdrop(&root);

  let init_campaign_metadata: Metadata = view!(
    near_campaign.get_campaign_metadata()
  ).unwrap_json();

  assert_eq!(init_campaign_metadata.social_network, "");

  let valid_call_result: ExecutionResult = call!(
    near_campaign.user_account,
    near_campaign.set_social_network("twitter".to_string())
  );

  assert!(valid_call_result.is_ok());

  let after_campaign_metadata: Metadata = view!(
    near_campaign.get_campaign_metadata()
  ).unwrap_json();

  assert_eq!(after_campaign_metadata.social_network, "twitter");
}
