use subxt::{balances::*, sp_core::crypto::AccountId32, ClientBuilder, Error, IndracoreRuntime};

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_total_issuance() {
        let total_issuance = get_total_issuance("ws://127.0.0.1:9944");
        assert_ne!(total_issuance.unwrap(), 0)
    }
}
