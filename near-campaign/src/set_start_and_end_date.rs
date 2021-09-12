use crate::*;
use near_sdk::{log, require};

#[near_bindgen]
impl Campaign {
  #[private]
  pub fn set_start_and_end_date(&mut self, start_date: Timestamp, end_date: Timestamp) {
    log!("Current start date {:#?} and end date {:#?} vs block timestamp {:#?}", self.start_date, self.end_date, env::block_timestamp());
    log!("Given start date {:#?} and end date {:#?}", start_date, end_date);

    require!(end_date > start_date, "Given end date is before start date");

    self.start_date = start_date;
    self.end_date = end_date;
  }
}
