use near_sdk::{env, near_bindgen, Promise};

mod create_user_account;

const USER_WASM: &[u8] = include_bytes!("../../target/wasm32-unknown-unknown/release/user.wasm");

#[near_bindgen]
pub struct Linkdrop {}
