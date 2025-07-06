use std::time::{SystemTime, UNIX_EPOCH};

use crate::rng::MersenneTwister64;

pub const MAGIC: u128 = 1_000_000_000;

pub fn get_time() -> u128 {
    MersenneTwister64::new(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64,
    )
    .next_u128()
}

pub fn wrap_number(x: u128, a: u128, b: u128) -> u128 {
    let range = b - a;
    let mut rem = (x - a) % range;
    rem += a;
    rem
}
