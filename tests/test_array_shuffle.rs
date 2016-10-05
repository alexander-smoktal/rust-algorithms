#[macro_use]
extern crate rust_algorithms;

use rust_algorithms::array_shuffle::ArrayShuffle;

#[test]
fn test_vector_shuffle() {
    let source_array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut copy_array = source_array.clone();

    copy_array.shuffle();

    assert!(source_array != copy_array);
}

#[test]
fn test_array_shuffle() {
    let source_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut copy_array = source_array.clone();

    copy_array.shuffle();

    assert!(source_array != copy_array);
}

#[test]
fn test_slice_shuffle() {
    let source_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut copy_array = source_array.clone();

    copy_array[3..7].shuffle();

    assert!(source_array != copy_array);
}
