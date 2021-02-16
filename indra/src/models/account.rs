use crate::models::balance::BalanceInfo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountInfo {
    pub accountid: Vec<u8>,
    pub balance: u128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub nonce: u32,
    pub refcount: u32,
    pub data: BalanceInfo,
}
