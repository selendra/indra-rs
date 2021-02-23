use indracore_subxt::{
    sp_core::{crypto, sr25519, Pair as TraitPair},
    sp_runtime, Error, IndracoreRuntime, PairSigner,
};
use std::str::FromStr;

type KusamaPairSigner = PairSigner<IndracoreRuntime, sr25519::Pair>;
type MultiAddressId = sp_runtime::MultiAddress<crypto::AccountId32, ()>;

#[derive(Clone)]
pub struct IndraKeyring {
    pairsigner: KusamaPairSigner,
    accountid: crypto::AccountId32,
}

impl IndraKeyring {
    pub fn from_str(mnemonic: &'static str) -> Result<IndraKeyring, Error> {
        match sr25519::Pair::from_string(&mnemonic, None) {
            Ok(pair) => {
                let pairsigner = PairSigner::<IndracoreRuntime, sr25519::Pair>::new(pair.clone());
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

    pub fn multi_address_from_str(accountid: &'static str) -> Result<MultiAddressId, Error> {
        match sp_runtime::AccountId32::from_str(accountid) {
            Ok(id) => Ok(sp_runtime::MultiAddress::from(id)),
            Err(e) => return Err(Error::Other(e.into())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_pair() {
        let mnemonic =
            "plug math bacon find roast scrap shrug exchange announce october exclude plate";
        let mnemonic_pair = IndraKeyring::from_str(mnemonic).unwrap();

        let seed = "0x9abdf3e8edda03c1708bcd5bc3353e91efd503fd9105ff0ee68a7cbc66b740d8";
        let seed_pair = IndraKeyring::from_str(seed).unwrap();

        assert_eq!(mnemonic_pair.accountid, seed_pair.accountid)
    }

    #[test]
    fn test_get_accountid_from_string() {
        let alice = "BauKu2iL4fncgfy22YSLGc1aDLpyuUUe5z8yNF2pDtLNr4E";
        let bob = "A1k3praCLftTgBTb6aVavh3UNKwXN599Fqov17MkEy6bwCU";
        let alice_id = IndraKeyring::accountid_from_str(alice);
        let bob_id = IndraKeyring::accountid_from_str(bob);
        assert_ne!(alice_id.unwrap(), bob_id.unwrap());
    }

    #[test]
    fn test_get_muti_address_from_string() {
        let alice = "BauKu2iL4fncgfy22YSLGc1aDLpyuUUe5z8yNF2pDtLNr4E";
        let bob = "A1k3praCLftTgBTb6aVavh3UNKwXN599Fqov17MkEy6bwCU";
        let alice_id = IndraKeyring::multi_address_from_str(alice);
        let bob_id = IndraKeyring::multi_address_from_str(bob);
        assert_ne!(alice_id.unwrap(), bob_id.unwrap());
    }
}
