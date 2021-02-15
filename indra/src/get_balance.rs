use subxt::{
    ClientBuilder,
    IndracoreRuntime,
    Error, balances::*,
};

pub fn get_total_issuance(url: String) -> Result<u128, Error> {
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
        Err(e)=> return Err(e)
    };
    Ok(total_issuance)
    })
}