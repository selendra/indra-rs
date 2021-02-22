use substrate_subxt::{balances::*, ClientBuilder, Error, KusamaRuntime, sp_core::H256};
use crate::utils::keyring::IndraKeyring;

#[derive(Debug)]
pub struct Transfer {
    url: &'static str,
    from: &'static str,
    to: &'static str,
    amount: u128,
}

impl Transfer {
    pub fn new(url: &'static str, from: &'static str, to: &'static str, amount: u128) -> Self {
        Self {url, from, to, amount }
    }

    pub fn transfer(&self) -> Result<H256, Error> {
        async_std::task::block_on(async move {
            let client = match ClientBuilder::<KusamaRuntime>::new()
                .set_url(self.url)
                .build()
                .await
            {
                Ok(c) => c,
                Err(e) => return Err(e),
            };
            let from = match IndraKeyring::from_mnemonic(self.from) {
                Ok(acc) => acc,
                Err(e) => return Err(e),
            };

            let signer = from.pairsingner();

            let dest = match IndraKeyring::accountid_from_str(self.to) {
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
