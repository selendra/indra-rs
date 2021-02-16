#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BalanceInfo {
    pub free: u128,
    pub reserved: u128,
    pub misc_frozen: u128,
    pub fee_frozen: u128,
}
