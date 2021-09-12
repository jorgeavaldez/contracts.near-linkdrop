use crate::*;
use near_sdk::log;

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn set_social_network(&mut self, network: String) {
    log!("Current social network {:#?} vs existing social network {:#?}", network, self.social_network);

    self.social_network = network;
  }
}
