extern crate rust_algorithms;

use rust_algorithms::array_shuffle;

#[test]
fn test_vector_shuffle() {
    let source_array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let shuffled_array = array_shuffle::shuffle(&source_array);

    assert!(source_array != shuffled_array);
}

#[test]
fn test_array_shuffle() {
    let source_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let shuffled_array = array_shuffle::shuffle(&source_array);

    assert!(source_array != shuffled_array);
}

