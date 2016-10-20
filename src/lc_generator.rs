/*******************************************************************************
 * LINEAR CONGRUENTIAL GENERATOR
 *
 * https://en.wikipedia.org/wiki/Linear_congruential_generator
 *
 ******************************************************************************/

use std::u64;
use std::time;
use std::sync::{Once, ONCE_INIT};

const A: u64 = 6364136223846793005;
const C: u64 = 1442695040888963407;
const MODULUS: u64 = u64::MAX;

static mut current_val: u64 = 0;

static ONCE: Once = ONCE_INIT;

pub trait Random {
    type ResultType;

    fn rnd() -> u64 {
        unsafe {
            ONCE.call_once(|| {
                match time::SystemTime::now().duration_since(time::UNIX_EPOCH) {
                    Ok(duration) => current_val = duration.as_secs(),
                    _ => current_val = 0
                }
            });

            current_val = (current_val.wrapping_mul(A).wrapping_add(C)) % MODULUS;
            current_val.clone()
        }
    }

     fn random() -> Self::ResultType;
}

impl Random for u64 {
    type ResultType = Self;

    fn random() -> Self {
        Self::rnd()
    }
}

impl Random for u32 {
    type ResultType = Self;

    fn random() -> Self {
        (Self::rnd() >> 32) as u32
    }
}

impl Random for u8 {
    type ResultType = Self;

    fn random() -> Self {
        (Self::rnd() >> 56) as u8
    }
}

impl Random for usize {
    type ResultType = Self;

    fn random() -> Self {
        (Self::rnd() >> (64 - usize::max_value().count_ones())) as usize
    }
}