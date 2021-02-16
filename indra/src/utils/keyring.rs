use crate::models::crypto::KeyPair;
use std::str::FromStr;
use subxt::{
    sp_core::{crypto, sr25519, Pair as TraitPair},
    sp_runtime, Error, IndracoreRuntime, PairSigner,
};

pub fn from_string_to_accountid(account_id: &str) -> Result<crypto::AccountId32, Error> {
    match sp_runtime::AccountId32::from_str(account_id) {
        Ok(id) => Ok(id),
        Err(e) => return Err(Error::Other(e.into())),
    }
}

pub fn get_keypair(phrase: &str) -> Result<KeyPair, Error> {
    match sr25519::Pair::from_string(&phrase, None) {
        Ok(p) => {
            let signer = PairSigner::<IndracoreRuntime, sr25519::Pair>::new(p.clone());
            let account_id = crypto::AccountId32::from(p.clone().public());
            Ok(KeyPair { signer, account_id })
        }
        Err(e) => Err(Error::Other(format!("{:?}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_accountid_from_string() {
        let alice = "BauKu2iL4fncgfy22YSLGc1aDLpyuUUe5z8yNF2pDtLNr4E";
        let bob = "A1k3praCLftTgBTb6aVavh3UNKwXN599Fqov17MkEy6bwCU";
        let alice_id = from_string_to_accountid(alice);
        let bob_id = from_string_to_accountid(bob);
        assert_ne!(alice_id.unwrap(), bob_id.unwrap());
    }

    #[test]
    fn test_keypair() {
        let phrase = "elbow crop stairs spy book stable merry lumber elite uncle evil shadow";
        let keypair = get_keypair(phrase).unwrap();

        let test_id = from_string_to_accountid("8ZkP4bcdb9yHZLRKH1soMSg19aHpuURAtuaAZETP6JHt7HU").unwrap();
        assert_eq!(keypair.account_id, test_id);
    }
}
