use crate::*;
use near_sdk::Timestamp;

// TODO add total keys are - we need to create a new campaign with a predefined amount of keys
// instead of calculated in dynamically during add_keys

#[near_bindgen]
impl Campaign {
  #[init]
  pub fn new(
    tokens_per_key: U128,
    start_date: Option<Timestamp>,
    end_date: Option<Timestamp>,
    social_network: Option<String>,
  ) -> Self {
    Self {
      tokens_per_key: tokens_per_key.into(),
      keys_stats: KeysStats {
        total: 0,
        active: 0,
        created: 0,
        claimed: 0,
        refunded: 0,
      },
      created_at: env::block_timestamp(),
      keys: UnorderedMap::new(b"k"),

      start_date: start_date.unwrap_or_default(),
      end_date: end_date.unwrap_or_default(),

      social_network: social_network.unwrap_or_default(),
    }
  }
}
