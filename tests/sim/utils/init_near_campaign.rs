use near_campaign::CampaignContract as NearCampaign;
use near_sdk::json_types::U128;
use near_sdk_sim::runtime::GenesisConfig;
use near_sdk_sim::{
  deploy, init_simulator, lazy_static_include, to_yocto, ContractAccount, UserAccount,
};

lazy_static_include::lazy_static_include_bytes! {
   NEAR_CAMPAIGN => "../target/wasm32-unknown-unknown/release/near_campaign.wasm"
}

pub fn init_near_campaign(tokens_per_key: &str) -> (UserAccount, ContractAccount<NearCampaign>) {
  let genesis = GenesisConfig::default();
  let root = init_simulator(Some(genesis));

  let near_campaign = deploy!(
    contract: NearCampaign,
    contract_id: "near_campaign",
    bytes: &NEAR_CAMPAIGN,
    signer_account: root,
    init_method: new(U128::from(to_yocto(tokens_per_key)), None, None, None)
  );

  (root, near_campaign)
}