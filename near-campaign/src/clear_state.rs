use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum ClearStatus {
  Completed(bool),
}

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn clear_state(&mut self, keys: Vec<PublicKey>) -> ClearStatus {
    keys.into_iter().for_each(|pk| {
      let key = pk.into();
      let key_status = self.keys.remove(&key);

      match key_status {
        Some(status) => { 
          if status == KeyStatus::Active {
            Promise::new(env::current_account_id()).delete_key(key);
          }
        },
        None => (),
      };
    });

    ClearStatus::Completed(self.keys.is_empty())
  }
}
