use subxt::{sp_runtime, Error};

use std::str::FromStr;

pub fn from_string_to_accountid(account_id: &str) -> Result<sp_core::crypto::AccountId32, Error> {
    match sp_runtime::AccountId32::from_str(account_id) {
        Ok(id) => Ok(id),
        Err(e) => return Err(Error::Other(e.into())),
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
}
