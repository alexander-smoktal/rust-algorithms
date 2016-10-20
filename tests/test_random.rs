#[macro_use]
extern crate rust_algorithms;

use rust_algorithms::lc_generator::Random;

#[test]
fn test_random() {
    let (val1, val2, val3) = (u64::random(), u64::random(), u64::random());

    println!("Random u64: {} {} {}", val1, val2, val3);

    assert!(val1 != val2);
    assert!(val3 != val2);
    assert!(val1 != val3);

    let (val4, val5, val6) = (u32::random(), u32::random(), u32::random());

    println!("Random u32: {} {} {}", val4, val5, val6);

    assert!(val4 != val5);
    assert!(val4 != val6);
    assert!(val5 != val6);

    let (val7, val8, val9) = (u8::random(), u8::random(), u8::random());

    println!("Random u8: {} {} {}", val7, val8, val9);

    assert!(val8 != val9);
    assert!(val7 != val9);
    assert!(val7 != val8);
}