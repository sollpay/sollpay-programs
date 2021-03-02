#[cfg(feature = "production")]
use std::env;

pub const SUBSCRIPTION_PLAN_SIZE: usize = 114;
pub const SUBSCRIPTION_SIZE: usize = 130;

#[cfg(feature = "production")]
const PROGRAM_OWNER_FEE_ADDRESS: &'static str = env!("PROGRAM_OWNER_FEE_ADDRESS");
