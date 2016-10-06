
pub struct ZipAll<T, U> where T: Iterator, U: Iterator {
    iter_left: T,
    iter_right: U
}

impl<T, U> Iterator for ZipAll<T, U> where T: Iterator, U: Iterator {
    type Item = (Option<T::Item>, Option<U::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.iter_left.next(), self.iter_right.next()) {
            (None, None) => None,
            pair => Some(pair)
        }
    }
}

pub fn zip_all<U>(left: U, right: U) -> ZipAll<U, U> where U: Iterator {
    ZipAll {
        iter_left: left,
        iter_right: right
    }
}