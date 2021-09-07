use crate::*;
use near_sdk::log;

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn add_keys(&mut self, keys: Vec<PublicKey>) -> u64 {
    log!("Current end date {:#?} vs block timestamp {:#?}", self.end_date, env::block_timestamp());

    if self.end_date != 0 && self.end_date < env::block_timestamp() {
      return 0;
    }


    let mut total_added = 0;

    keys.into_iter().for_each(|key| {
      if self.keys.get(&key).is_none() {
        self.keys.insert(&key, &KeyStatus::Active);
        self.keys_stats.total += 1;
        self.keys_stats.active += 1;
        total_added += 1;

        Promise::new(env::current_account_id()).add_access_key(
          key,
          1_000_000_000_000_000_000_000_000, // 1 NEAR TODO Should we use an unlimited amount?
          env::current_account_id(),
          "create_account_and_claim,claim".to_string(),
        );
      }
    });

    total_added
  }
}
