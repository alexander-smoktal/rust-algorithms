extern crate rust_algorithms;

use rust_algorithms::datatypes::bitset::BitSet;
use std::iter::FromIterator;

#[test]
#[should_panic]
fn test_invalid_constructor() {
    let _ = BitSet::from_iter(&[1, 0, 1, 1, 0, 0, 7]);
}

#[test]
fn test_set_get() {
    let mut set = BitSet::new(16);

    assert_eq!(set.bit(0), 0);
    assert_eq!(set.bit(5), 0);
    assert_eq!(set.bit(13), 0);

    set.set_bit(5, 0);
    set.set_bit(7, 1);
    set.set_bit(14, 1);

    assert_eq!(set.bit(0), 0);
    assert_eq!(set.bit(5), 0);
    assert_eq!(set.bit(7), 1);
    assert_eq!(set.bit(13), 0);
    assert_eq!(set.bit(14), 1);
}

#[test]
#[should_panic]
fn test_oob_set() {
    let mut set = BitSet::new(5);
    set.set_bit(5, 0);
}

#[test]
#[should_panic]
fn test_invalid_set() {
    let mut set = BitSet::new(5);
    set.set_bit(4, 3);
}

#[test]
#[should_panic]
fn test_oob_get() {
    let set = BitSet::new(5);
    set.bit(7);
}

#[test]
fn test_array_constructor() {
    let set = BitSet::from_iter(&[1, 0, 1, 1, 0, 0, 1]);

    assert_eq!(set.bit(0), 1);
    assert_eq!(set.bit(1), 0);
    assert_eq!(set.bit(2), 1);
    assert_eq!(set.bit(3), 1);
    assert_eq!(set.bit(4), 0);
    assert_eq!(set.bit(5), 0);
    assert_eq!(set.bit(6), 1);
}

#[test]
fn test_equal() {
    let set1 = BitSet::from_iter(&[1, 0, 1, 1, 0, 0, 1]);
    let set2 = BitSet::from_iter(&[1, 0, 1, 1, 0, 0, 1]);
    let set3 = BitSet::from_iter(&[1, 0, 1, 0, 1, 0, 1]);

    assert_eq!(set1, set2);
    assert!(set1 != set3);
    assert!(set2 != set3);
}


#[test]
fn test_ordering() {
    let set1 = BitSet::from_iter(&[1, 0, 0, 0, 0, 0, 0]);
    let set2 = BitSet::from_iter(&[0, 1]);
    let set3 = BitSet::from_iter(&[1, 0, 1, 0, 1, 0, 0]);

    assert!(set1 < set2);
    assert!(set1 < set3);
    assert!(set3 > set2);
}

#[test]
fn test_binops() {
    let set_left =  BitSet::from_iter(&[1, 0, 1, 0, 1, 0, 1]);
    let ref set_right = BitSet::from_iter(&[1, 0, 0, 1, 0, 0, 1]);

    assert_eq!(set_left.not(), BitSet::from_iter(&[0, 1, 0, 1, 0, 1, 0]));
    assert_eq!(set_left.or(set_right), BitSet::from_iter(&[1, 0, 1, 1, 1, 0, 1]));
    assert_eq!(set_left.and(set_right), BitSet::from_iter(&[1, 0, 0, 0, 0, 0, 1]));
    assert_eq!(set_left.xor(set_right), BitSet::from_iter(&[0, 0, 1, 1, 1, 0, 0]));
}