extern crate rust_algorithms;

use rust_algorithms::prime_test::{TrialPrime, Prime};

const PRIMES: [u32; 7] = [2, 3, 7, 23, 59, 151, 1439];
const NOT_PRIMES: [u32; 11] = [0, 1, 4, 6, 9, 10, 18, 28, 60, 152, 1130];

#[test]
fn test_types() {
    assert!(!1u8.is_prime_trail());
    assert!(!1u16.is_prime_trail());
    assert!(!1u32.is_prime_trail());
    assert!(!1u64.is_prime_trail());

    assert!(!1u8.is_prime());
    assert!(!1u16.is_prime());
    assert!(!1u32.is_prime());
    assert!(!1u64.is_prime());
}

#[test]
fn test_primes_trial() {
    for n in PRIMES.into_iter() {
        assert!(n.is_prime_trail());
    }

    for n in NOT_PRIMES.into_iter() {
        assert!(!n.is_prime_trail());
    }
}

#[test]
fn test_primes() {
    for n in PRIMES.into_iter() {
        assert!(n.is_prime());
    }

    for n in NOT_PRIMES.into_iter() {
        assert!(!n.is_prime());
    }
}

#[test]
fn test_intersection() {
    for n in 0..1000u32 {
        assert!(n.is_prime_trail() == n.is_prime(),
                "Functions results don't match for {}", n)
    }
}