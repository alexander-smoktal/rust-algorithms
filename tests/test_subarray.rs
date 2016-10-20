extern crate rust_algorithms;

use rust_algorithms::max_subarray::MaxSubarray;

#[test]
fn test_subarray() {
    let start = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];

    println!("Subarray {:?}", start.max_subarray());

    assert_eq!(start.max_subarray(), &[4, -1, 2, 1]);
    assert_eq!(vec![8].max_subarray(), &[8]);
    assert_eq!(Vec::<i8>::new().max_subarray(), &[]);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8].max_subarray(), &[1, 2, 3, 4, 5, 6, 7, 8]);
}