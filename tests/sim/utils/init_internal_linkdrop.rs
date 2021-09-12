use linkdrop::LinkdropContract as Linkdrop;
use near_sdk_sim::runtime::GenesisConfig;
use near_sdk_sim::{
  deploy, init_simulator, lazy_static_include, ContractAccount, UserAccount
};

lazy_static_include::lazy_static_include_bytes! {
   INTERNAL_LINKDROP => "../target/wasm32-unknown-unknown/release/linkdrop.wasm"
}

pub fn init_internal_linkdrop() -> (UserAccount, ContractAccount<Linkdrop>) {
  let genesis = GenesisConfig::default();
  let root = init_simulator(Some(genesis));

  let linkdrop = deploy!(
    contract: Linkdrop,
    contract_id: "linkdrop",
    bytes: &INTERNAL_LINKDROP,
    signer_account: root,
  );

  (root, linkdrop)
}