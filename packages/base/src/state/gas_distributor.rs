use crate::msg::gas_distributor::TargetBalance;
use cw_storage_plus::Map;

pub static UNTRN_DENOM: &str = "untrn";
pub const TARGET_BALANCES: Map<String, TargetBalance> = Map::new("target_balances");
