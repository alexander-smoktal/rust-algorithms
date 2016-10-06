extern crate rust_algorithms;

use rust_algorithms::datatypes::matrix::{Matrix};

macro_rules! matrix {
    ($elem:expr; $width:expr, $height:expr) => {
        Matrix {
            data: vec![$elem; $width * $height],
            width: $width,
            height: $height
        }
    };
    ($($elem:expr),+) => ({
        let mut result: Vec<i64> = Vec::new();
        let mut len = usize::max_value();
        
        $(
            if len == usize::max_value() { len = $elem.len() }
            assert!($elem.len() == len, "Matrix rows have different len: {} and {}", $elem.len(), len);

            result.extend($elem.into_iter());
        )*
        
        Matrix {
            width: len,
            height: result.len() / len,
            data: result
        }
    });
}

#[test]
fn test_constructor() {
    let mat0 = Matrix::<i64>::new(4, 6);
    println!("Matrix0: {:?}", mat0);

    let mat1 = matrix![3; 3, 2];
    println!("Matrix1: {:?}", mat1);

    let mat2 = matrix![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("Matrix2: {:?}", mat2);
}