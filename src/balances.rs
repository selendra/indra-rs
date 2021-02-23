use crate::utils::keyring::IndraKeyring;
use indracore_subxt::{balances::*, system::*, ClientBuilder, Error, IndracoreRuntime};

pub struct Balance {}

impl Balance {
    pub fn get_total_issuance(url: &'static str) -> Result<u128, Error> {
        async_std::task::block_on(async move {
            let client = match ClientBuilder::<IndracoreRuntime>::new()
                .set_url(url)
                .build()
                .await
            {
                Ok(c) => c,
                Err(e) => return Err(e),
            };
            let total_issuance = match client.total_issuance(None).await {
                Ok(t) => t,
                Err(e) => return Err(e),
            };
            Ok(total_issuance)
        })
    }

    pub fn get_free_balance(url: &'static str, account_id: &'static str) -> Result<u128, Error> {
        async_std::task::block_on(async move {
            let client = match ClientBuilder::<IndracoreRuntime>::new()
                .set_url(url)
                .build()
                .await
            {
                Ok(c) => c,
                Err(e) => return Err(e),
            };
            let accountid32 = match IndraKeyring::accountid_from_str(account_id) {
                Ok(acc) => acc,
                Err(e) => return Err(e),
            };
            let info = match client.account(&accountid32, None).await {
                Ok(i) => i,
                Err(e) => return Err(e),
            };
            Ok(info.data.free)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_total_issuance() {
        let url = "ws://127.0.0.1:9944";
        let total_issuance = Balance::get_total_issuance(url);
        assert_ne!(total_issuance.unwrap(), 0)
    }

    #[test]
    fn test_state_read_free_balance() {
        let account_id = "BauKu2iL4fncgfy22YSLGc1aDLpyuUUe5z8yNF2pDtLNr4E";
        let url = "ws://127.0.0.1:9944";
        let balance = Balance::get_free_balance(url, account_id);
        assert_ne!(balance.unwrap(), 0);
    }
}
