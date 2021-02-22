use crate::utils::keyring::IndraKeyring;
use substrate_subxt::{system::AccountStoreExt, ClientBuilder, Error, IndracoreRuntime};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BalanceInfo {
    pub free: u128,
    pub reserved: u128,
    pub misc_frozen: u128,
    pub fee_frozen: u128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountInfo {
    pub nonce: u32,
    pub refcount: u32,
    pub data: BalanceInfo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllAccount {
    pub accountid: Vec<u8>,
    pub balance: u128,
}

pub struct Account {}

impl Account {
    pub fn fecth_all_account(url: &'static str) -> Result<Vec<AllAccount>, Error> {
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
            let mut account_data: Vec<AllAccount> = Vec::new();
            while let Some((key, account)) = iter.next().await? {
                let data = AllAccount {
                    accountid: key.0,
                    balance: account.data.free,
                };
                account_data.push(data);
            }
            Ok(account_data)
        })
    }

    pub fn get_account_info(url: &str, account_id: &'static str) -> Result<AccountInfo, Error> {
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
            let data = BalanceInfo {
                free: info.data.free,
                reserved: info.data.reserved,
                misc_frozen: info.data.misc_frozen,
                fee_frozen: info.data.fee_frozen,
            };
            Ok(AccountInfo {
                nonce: info.nonce,
                refcount: info.refcount,
                data,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fecth_all_account() {
        assert!(Account::fecth_all_account("ws://127.0.0.1:9944").is_ok())
    }

    #[test]
    fn test_fecth_account_info() {
        let account_id = "BauKu2iL4fncgfy22YSLGc1aDLpyuUUe5z8yNF2pDtLNr4E";
        let info = Account::get_account_info("ws://127.0.0.1:9944", account_id);

        let balance_info = BalanceInfo {
            free: 0,
            reserved: 0,
            misc_frozen: 0,
            fee_frozen: 0,
        };
        let test_info = AccountInfo {
            nonce: 0,
            refcount: 0,
            data: balance_info,
        };
        assert_ne!(info.unwrap(), test_info);
    }
}
