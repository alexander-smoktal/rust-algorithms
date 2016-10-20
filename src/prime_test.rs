/*******************************************************************************
 *
 * http://en.wikipedia.org/wiki/Primality_test
 * http://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test
 *
 ******************************************************************************/
use math;
use lc_generator::Random;

/* Trial primality test */
pub trait TrialPrime where u64: From<Self>, Self: Sized {
    fn is_prime_trail(self) -> bool {
        let value = u64::from(self);

        match value {
            0 | 1 => false,
            2 => true,
            n => {
                for i in 2..(n/2 + 1) {
                    if n % i == 0 { return false }
                }
                true
            }
        }
    }
}

impl TrialPrime for u64 {}
impl TrialPrime for u32 {}
impl TrialPrime for u16 {}
impl TrialPrime for u8 {}

/* Miller–Rabin primality test.
 * All one-letter variables come from the algorithm description. Just
 * read wiki article, to anderstand, what's going on. 
 * TODO: Move to trait, when #29646 fixed (Allow to use const in a trait) */
const ACCURACY: usize = 5;
pub trait Prime where u64: From<Self>, Self: Sized {
    fn is_prime(self) -> bool {
        let value = u64::from(self);

        match value {
            0 | 1 => false,
            2 | 3 => true,
            n if n % 2 == 0 => { false },
            n => {
                let s = (n - 1).trailing_zeros();
                let d = (n - 1) >> s;

                'witness: for _ in 0..ACCURACY {
                    let a = u64::random() % (n - 4) + 2;
                    let mut x = math::mod_exp(a, d, n); // NOTE: we use own function for modular exponentiation

                    if x == 1 || x == (n - 1) { continue }

                    for _ in 1..s {
                        x = math::mod_exp(x, 2, n);
                        if x == 1 { return false }
                        else if x == (n - 1) { continue 'witness }
                    }
                    return false
                }
                true
            }
        }
    }
}

impl Prime for u64 {}
impl Prime for u32 {}
impl Prime for u16 {}
impl Prime for u8 {}