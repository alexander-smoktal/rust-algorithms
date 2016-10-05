#[macro_export]
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
            assert!($elem.len() != len, "Matrix rows have different len");

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

impl<T> Matrix<T> {
     pub fn new(width: usize, height: usize) -> Matrix<T> {
        assert!(width * height < usize::max_value());

        Matrix::<T> {
            data: Vec::with_capacity(width * height),
            width: width,
            height: height,
        }
    }

    pub fn get<'a>(&'a self, row: usize, column: usize) -> &'a T {
        assert!(row < self.height, "Row index is out of bounds. Index: {}, height: {}", row, self.height);
        assert!(column < self.width, "Column index is out of bounds. Index: {}, width: {}", column, self.width);

        &self.data[row * self.height + column]
    }

    pub fn get_mut<'a>(&'a mut self, row: usize, column: usize) -> &'a mut T {
        assert!(row < self.height, "Row index is out of bounds. Index: {}, height: {}", row, self.height);
        assert!(column < self.width, "Column index is out of bounds. Index: {}, width: {}", column, self.width);

        &mut self.data[row * self.height + column]
    }
}
