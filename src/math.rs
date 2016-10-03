
/* Effiective Modular Exponention function
 * https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method */
pub fn mod_exp(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 { return 0 }

    assert!(modulus - 1 <= u64::max_value() / 2, "Too big modulus: {}", modulus);

    let mut result = 1u64;

    base = base % modulus;
    while exponent > 0 {
        if exponent % 2 == 1 { result = (result * base) % modulus }
        exponent = exponent >> 1;
        base = (base * base) % modulus
    }
    return result
}