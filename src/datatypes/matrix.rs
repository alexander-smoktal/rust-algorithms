/******************************************************************************
 * MTRIX IMPLEMENTATION
 *
 * https://en.wikipedia.org/wiki/Matrix_(mathematics)
 * 
 ******************************************************************************/

use std::cmp::PartialEq;
use std::ops::{Mul, Add};

extern crate num;

#[macro_export]
macro_rules! matrix {
    ($elem:expr; $height:expr, $width:expr) => {
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
            assert!($elem.len() == len, "Matrix rows have different len. Current row len: {}, previous rows len: {}",
                    $elem.len(), len);

            result.extend($elem.into_iter());
        )*

        Matrix {
            width: len,
            height: result.len() / len,
            data: result
        }
    });
    // TODO: Use intristics when stable
    ($typ:ty => $($elem:expr),+) => ({
        let mut result: Vec<$typ> = Vec::new();
        let mut len = usize::max_value();
        
        $(
            if len == usize::max_value() { len = $elem.len() }
            assert!($elem.len() == len, "Matrix rows have different len. Current row len: {}, previous rows len: {}",
                    $elem.len(), len);

            result.extend($elem.into_iter());
        )*

        Matrix {
            width: len,
            height: result.len() / len,
            data: result
        }
    });
}

// TODO: use heap::*, when stable
#[derive(Debug)]
pub struct Matrix<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

/// _Note_: Sorry, have no time to count determinant
impl<T> Matrix<T> where T: Copy + num::Zero {
     pub fn new(height: usize, width: usize) -> Matrix<T> {
        assert!(width * height < usize::max_value());

        Matrix::<T> {
            data: vec![T::zero(); width * height],
            width: width,
            height: height,
        }
    }

    pub fn get(&self, row: usize, column: usize) -> T {
        assert!(row < self.height, "Row index is out of bounds. Index: {}, height: {}", row, self.height);
        assert!(column < self.width, "Column index is out of bounds. Index: {}, width: {}", column, self.width);

        self.data[row * self.width + column]
    }

    pub fn set(&mut self, row: usize, column: usize, value: T) {
        assert!(row < self.height, "Row index is out of bounds. Index: {}, height: {}", row, self.height);
        assert!(column < self.width, "Column index is out of bounds. Index: {}, width: {}", column, self.width);

        self.data[row * self.width + column] = value
    }

    pub fn transpose(&self) -> Matrix<T> {
        let mut result = Matrix::new(self.width, self.height);

        for i in 0..self.width {
            for j in 0..self.height {
                result.set(i, j, self.get(j, i))
            }
        }
        result
    }
}

impl Matrix<f64> {
    /// Yeah, ol' good determinant
    pub fn determinant(&self) -> f64 {
        assert_eq!(self.width, self.height);

        let mut subi: usize;
        let mut subj: usize;
        let mut det = 0.0;
        if self.width == 1 { det = f64::from(self.data[0]) }
        else if self.width == 2 { det = f64::from(self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0)) } 
        else {
            let mut mat = Matrix::new(self.width - 1, self.height - 1);
            for c in 0..self.width {
                subi = 0;
                for i in 1..self.width {
                    subj = 0;
                    for j in 0..self.height{
                        if c == j { continue }
                        mat.set(subi, subj, self.get(i, j));
                        subj += 1
                    }
                    subi += 1
                }
                det += (if c % 2 == 0 { 1.0 } else { -1.0 }) * self.get(0, c) * mat.determinant();
            }
        }
        det
    }
}

impl<T, O> PartialEq<Matrix<O>> for Matrix<T> where T: PartialEq<O>  {
    fn eq(&self, other: &Matrix<O>) -> bool {
        self.width == other.width && self.height == other.height && self.data.as_slice() == other.data.as_slice()
    }
}

impl<T, O> Add<Matrix<O>> for Matrix<T> 
    where T: Add<O> + Copy + num::Zero,
          O: Copy + num::Zero,
          <T as Add<O>>::Output: Copy + num::Zero {
    type Output = Matrix<<T as Add<O>>::Output>;

    fn add(self, rhs: Matrix<O>) -> Self::Output {
        assert_eq!(self.width, rhs.width);
        assert_eq!(self.height, rhs.height);

        let mut result = Matrix::new(self.height, self.width);

        for i in 0..self.width {
            for j in 0..self.height {
                result.set(j, i, self.get(j, i) + rhs.get(j, i))
            }
        }
        result
    }
}

impl<T, O> Mul<Matrix<O>> for Matrix<T> 
    where T: Mul<O> + Copy + num::Zero,
          O: Copy + num::Zero,
          <T as Mul<O>>::Output: Copy + num::Zero {
    type Output = Matrix<<T as Mul<O>>::Output>;

    fn mul(self, rhs: Matrix<O>) -> Self::Output { 
        assert_eq!(self.width, rhs.height);
        assert_eq!(self.height, rhs.width);

        let mut result = Matrix::new(self.height, self.width);

        for i in 0..self.width {
            for j in 0..self.height {
                result.set(j, i, self.get(j, i) * rhs.get(i, j))
            }
        }
        result
    }
}

impl<T, O> Mul<O> for Matrix<T> 
    where T: Mul<O> + Copy + num::Zero,
          O: Copy + num::Zero,
          <T as Mul<O>>::Output: Copy + num::Zero {
    type Output = Matrix<<T as Mul<O>>::Output>;

    fn mul(self, rhs: O) -> Self::Output { 
        let mut result = Matrix::new(self.height, self.width);

        for i in 0..self.width {
            for j in 0..self.height {
                result.set(j, i, self.get(j, i) * rhs)
            }
        }
        result
    }
}