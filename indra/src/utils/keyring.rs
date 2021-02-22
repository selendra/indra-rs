use std::str::FromStr;
use substrate_subxt::{
    sp_core::{crypto, sr25519, Pair as TraitPair},
    sp_runtime, Error, KusamaRuntime, PairSigner,
};

type KusamaPairSigner = PairSigner<KusamaRuntime, sr25519::Pair>;

#[derive(Clone)]
pub struct IndraKeyring {
    pairsigner: KusamaPairSigner,
    accountid: crypto::AccountId32,
}

impl IndraKeyring {
    pub fn from_mnemonic(mnemonic: &'static str) -> Result<IndraKeyring, Error> {
        match sr25519::Pair::from_string(&mnemonic, None) {
            Ok(pair) => {
                let pairsigner = PairSigner::<KusamaRuntime, sr25519::Pair>::new(pair.clone());
                let accountid = crypto::AccountId32::from(pair.clone().public());
                Ok(Self {
                    pairsigner,
                    accountid,
                })
            }
            Err(e) => Err(Error::Other(format!("{:?}", e))),
        }
    }

    pub fn pairsingner(&self) -> KusamaPairSigner {
        self.pairsigner.clone()
    }

    pub fn accountid(&self) -> crypto::AccountId32 {
        self.accountid.clone()
    }

    pub fn accountid_from_str(accountid: &'static str) -> Result<crypto::AccountId32, Error> {
        match sp_runtime::AccountId32::from_str(accountid) {
            Ok(id) => Ok(id),
            Err(e) => return Err(Error::Other(e.into())),
        }
    }
}
