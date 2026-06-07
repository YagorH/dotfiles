use std::time::{SystemTime, UNIX_EPOCH};

pub fn gen_range(min: u32, max: u32) -> u32 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    ((nanos % (max - min + 1) as u128) as u32) + min
}
