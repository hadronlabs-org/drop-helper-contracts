use crate::msg::gas_distributor::TargetBalance;
use cw_storage_plus::Item;

pub static UNTRN_DENOM: &str = "untrn";
pub const TARGET_BALANCES: Item<Vec<TargetBalance>> = Item::new("target_balances");
