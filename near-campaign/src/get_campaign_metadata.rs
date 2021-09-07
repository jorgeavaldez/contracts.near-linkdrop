use crate::*;

// View method
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
  tokens_per_key: U128,
  keys_stats: KeysStats,
  created_at: u64,
  status: String,
  start_date: u64,
  end_date: u64,
}

#[near_bindgen]
impl Campaign {
  pub fn get_campaign_metadata(self) -> Metadata {
    // Adding them here since active could get out of sync
    let inactive_key_total = self.keys_stats.claimed + self.keys_stats.refunded + self.keys_stats.created;

    let status = if self.keys_stats.total > inactive_key_total {
      "active"
    } else {
      "completed"
    };


    Metadata {
      tokens_per_key: self.tokens_per_key.into(),
      keys_stats: self.keys_stats,
      created_at: self.created_at,
      status: status.to_string(),
      start_date: self.start_date,
      end_date: self.end_date,
    }
  }
}
