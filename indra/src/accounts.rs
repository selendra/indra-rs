use crate::models::account::AccountInfo;
use subxt::{system::AccountStoreExt, ClientBuilder, Error, IndracoreRuntime};

pub fn fetch_all_accounts(url: &str) -> Result<Vec<AccountInfo>, Error> {
    async_std::task::block_on(async move {
        let client = match ClientBuilder::<IndracoreRuntime>::new()
            .set_url(url)
            .build()
            .await
        {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        let mut iter = match client.account_iter(None).await {
            Ok(i) => i,
            Err(e) => return Err(e),
        };
        let mut account_data: Vec<AccountInfo> = Vec::new();
        while let Some((key, account)) = match iter.next().await {
            Ok(i) => i,
            Err(e) => return Err(e),
        } {
            let balance = account.data.free;
            let data = AccountInfo {
                accountid: key.0,
                balance,
            };
            account_data.push(data);
        }
        Ok(account_data)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fect_all_account() {
        assert!(fetch_all_accounts("ws://127.0.0.1:9944").is_ok())
    }
}
