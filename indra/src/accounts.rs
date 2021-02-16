use crate::{
    models::{
        account::{Account, AccountInfo},
        balance::BalanceInfo,
    },
    utils::keyring::from_string_to_accountid,
};
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

pub fn get_account_info(url: &str, account_id: &str) -> Result<Account, Error> {
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
        let data = BalanceInfo {
            free: info.data.free,
            reserved: info.data.reserved,
            misc_frozen: info.data.misc_frozen,
            fee_frozen: info.data.fee_frozen,
        };
        Ok(Account {
            nonce: info.nonce,
            refcount: info.refcount,
            data,
        })
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
