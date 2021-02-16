use subxt::{sp_core, IndracoreRuntime, PairSigner};

pub type Sr25519 = PairSigner<IndracoreRuntime, sp_core::sr25519::Pair>;

#[derive(Clone)]
pub struct KeyPair {
    pub signer: Sr25519,
    pub account_id: sp_core::crypto::AccountId32,
}
