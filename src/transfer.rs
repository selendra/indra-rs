use crate::utils::keyring::IndraKeyring;
use indracore_subxt::{balances::*, sp_core::H256, ClientBuilder, Error, IndracoreRuntime};

#[derive(Debug)]
pub struct Transfer {
    url: &'static str,
    from: &'static str,
    to: &'static str,
    amount: u128,
}

impl Transfer {
    pub fn new(url: &'static str, from: &'static str, to: &'static str, amount: u128) -> Self {
        Self {
            url,
            from,
            to,
            amount,
        }
    }

    pub fn transfer(&self) -> Result<H256, Error> {
        async_std::task::block_on(async move {
            let client = match ClientBuilder::<IndracoreRuntime>::new()
                .set_url(self.url)
                .build()
                .await
            {
                Ok(c) => c,
                Err(e) => return Err(e),
            };
            let from = match IndraKeyring::from_str(self.from) {
                Ok(acc) => acc,
                Err(e) => return Err(e),
            };

            let signer = from.pairsingner();

            let dest = match IndraKeyring::multi_address_from_str(self.to) {
                Ok(to) => to,
                Err(e) => return Err(e),
            };

            let hash = match client.transfer(&signer, &dest, self.amount).await {
                Ok(h) => h,
                Err(e) => return Err(e),
            };
            Ok(hash)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transfer() {
        let to = "5FTussKPbMZReFdvL7tu97XzQe7uGqHtcH1PwKN9xCBN2WJ7";
        let from = "0x9abdf3e8edda03c1708bcd5bc3353e91efd503fd9105ff0ee68a7cbc66b740d8";
        let url = "ws://127.0.0.1:9944";
        let data = Transfer::new(url, from, to, 10_0000);
        assert!(data.transfer().is_err());
    }
}
