use subxt::{system::AccountStoreExt, ClientBuilder, Error, IndracoreRuntime};

pub fn fetch_all_accounts(url: String) -> Result<(), Error> {
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
        while let Some((key, account)) = match iter.next().await {
            Ok(i) => i,
            Err(e) => return Err(e),
        } {
            println!("{:?}: {}", key, account.data.free);
        }
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fect_all_account(){
        assert!(fetch_all_accounts("ws://127.0.0.1:9944".to_string()).is_ok())
    }
    
}