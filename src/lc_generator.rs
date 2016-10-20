use std::u64;

const A: u64 = 6364136223846793005;
const C: u64 = 1442695040888963407;
const MODULUS: u64 = u64::MAX;

static mut current_val: u64 = 0;

pub trait Random {
    type ResultType;

    fn rnd() -> u64 {
        unsafe {
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