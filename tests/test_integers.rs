extern crate rust_algorithms;

use rust_algorithms::datatypes::integer::Integer;

#[test]
fn test_integer_constructors() {
    let int = Integer::from_string("123".to_owned());
    println!("Integer: {} {:?}", int, int);

    let int = Integer::from_string("12345".to_owned());
    println!("Integer: {} {:?}", int, int);

    let int = Integer::from_string("-12345".to_owned());
    println!("Integer: {} {:?}", int, int);
}

#[test]
fn test_integer_sum_with_int() {
    println!("Integer: {}", Integer::from_string("123".to_owned()) + 15_u8);

    println!("Integer: {}", Integer::from_string("123".to_owned()) + 75_u8);

    println!("Integer: {}", Integer::from_string("345".to_owned()) + 12000_u32);

    println!("Integer: {:?}", Integer::from_string("71".to_owned()) + 131000_u32);

    println!("Integer: {:?}", Integer::from_string("5".to_owned()) + 16711930_u32);
}

// #[test]
// fn test_integer_mul_with_int() {
//     println!("Integer: {:?}", Integer::from_string("16".to_owned()) * 16_u32);

//     println!("Integer: {:?}", Integer::from_string("5".to_owned()) * 3342385_u64);
// }