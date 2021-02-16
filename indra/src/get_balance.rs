use crate::utils::keyring::from_string_to_accountid;
use subxt::{balances::*, system::*, ClientBuilder, Error, IndracoreRuntime};

pub fn get_total_issuance(url: &str) -> Result<u128, Error> {
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

pub fn get_free_balance(url: &str, account_id: &str) -> Result<u128, Error> {
    async_std::task::block_on(async move {
        let client = match ClientBuilder::<IndracoreRuntime>::new()
            .set_url(url)
            .build()
            .await
        {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        let accountid32 = match from_string_to_accountid(account_id) {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_total_issuance() {
        let total_issuance = get_total_issuance("ws://127.0.0.1:9944");
        assert_ne!(total_issuance.unwrap(), 0)
    }

    #[test]
    fn test_state_read_free_balance() {
        let account_id = "BauKu2iL4fncgfy22YSLGc1aDLpyuUUe5z8yNF2pDtLNr4E";
        let balance = get_free_balance("ws://127.0.0.1:9944", account_id);
        assert_ne!(balance.unwrap(), 0);
    }
}
