#[macro_use]
extern crate rust_algorithms;

use rust_algorithms::datatypes::matrix::{Matrix};

#[test]
fn test_constructor() {
    let mat0 = Matrix::<i64>::new(4, 6);
    println!("Matrix0: {:?}", mat0);

    let mat1 = matrix![3; 3, 2];
    println!("Matrix1: {:?}", mat1);

    let mat2 = matrix![[1, 2, 3],
                       [4, 5, 6],
                       [7, 8, 9]];
    println!("Matrix2: {:?}", mat2);
}

#[test]
fn test_eq() {
    assert_eq!(Matrix::<i64>::new(2, 3), matrix![[0, 0, 0],
                                                 [0, 0, 0]]);

    assert_eq!(matrix![3; 3, 2], matrix![[3, 3], [3, 3], [3, 3]]);
}

#[test]
#[should_panic]
fn test_invalid_mul() {
    let _ = matrix![3; 3, 2] * matrix![3; 3, 2];
}

#[test]
fn test_mul() {
    assert_eq!(matrix![3; 3, 2] * matrix![3; 2, 3], matrix![9; 3, 2]);
    assert_eq!(matrix![[3, 3], [3, 3], [3, 3]] * matrix![[1, 2, 3], [4, 5, 6]], matrix![[3, 12], [6, 15], [9, 18]]);
}

#[test]
#[should_panic]
fn test_invalid_add() {
    let _ = matrix![3; 3, 2] + matrix![3; 2, 3];
}

#[test]
fn test_add() {
    assert_eq!(matrix![3; 3, 2] + matrix![3; 3, 2], matrix![6; 3, 2]);
    assert_eq!(matrix![[3, 3], [3, 3], [3, 3]] + matrix![[1, 2], [3, 4], [5, 6]], matrix![[4, 5], [6, 7], [8, 9]]);
}
