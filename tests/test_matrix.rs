#[macro_use]
extern crate rust_algorithms;

use rust_algorithms::datatypes::matrix::{Matrix};

#[test]
fn test_matrix_constructor() {
    let mat0 = Matrix::<i64>::new(4, 6);
    println!("Matrix0: {:?}", mat0);

    let mat1 = matrix![3; 3, 2];
    println!("Matrix1: {:?}", mat1);

    let mat2 = matrix![[1, 2, 3],
                       [4, 5, 6],
                       [7, 8, 9]];
    println!("Matrix2: {:?}", mat2);

    let mat3 = matrix![f64 => [6.0, 3.0, 9.0],
                              [4.3, 52.0, 16.0],
                              [73.0, 8.2, 9.0]];
    println!("Matrix3: {:?}", mat3);
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
fn test_scalar_mul() {
    assert_eq!(matrix![3; 3, 2] * 2, matrix![6; 3, 2]);
    assert_eq!(matrix![[1, 2], [3, 4], [5, 6]] * 2, matrix![[2, 4], [6, 8], [10, 12]]);
}

#[test]
fn test_add() {
    assert_eq!(matrix![3; 3, 2] + matrix![3; 3, 2], matrix![6; 3, 2]);
    assert_eq!(matrix![[3, 3], [3, 3], [3, 3]] + matrix![[1, 2], [3, 4], [5, 6]], matrix![[4, 5], [6, 7], [8, 9]]);
}

#[test]
fn test_transpose() {
    assert_eq!(matrix![3; 3, 2].transpose(), matrix![3; 2, 3]);
    assert_eq!(matrix![[1, 2], [3, 4], [5, 6]].transpose(), matrix![[1, 3, 5], [2, 4, 6]]);
}

#[test]
fn test_determinant() {
    assert_eq!(matrix![7_f64; 1, 1].determinant(), 7.0);
    let mat = matrix![f64 => [1.0, 2.0],
                             [3.0, 4.0]];

    assert_eq!(mat.determinant(), -2.0);

    let _mat = matrix![f64 => [5.0, 6.0, 2.0, 13.0],
                              [5.0, 2.0, 4.0, 5.0],
                              [1.0, 3.0, 4.0, 5.0],
                              [2.0, 3.0, 4.0, 8.0]];
    assert_eq!(_mat.determinant(), -228.0);

    let mat = matrix![f64 => [1.0, -2.0, 3.0],
                           [4.0, 0.0, 6.0],
                           [-7.0, 8.0, 9.0]];
    assert_eq!(mat.determinant(), 204.0);
}
