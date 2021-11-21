use lazy_static::lazy_static;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod extension;

lazy_static! {
    pub static ref PROCESS_ID: u128 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
}
