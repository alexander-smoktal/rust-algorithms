//! ZipAll module.
//!
//! Provides functions, to zip iterators, until at least one of them returns Some(value)

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

/// Zip two iterators and return pairs, until any of iterator returns Some(value)
/// __Returns:__ Iterator for pairs of options
pub fn zip_all<T, E>(left: T, right: T) -> ZipAll<T::IntoIter, T::IntoIter> where T: IntoIterator<Item=E> {
    ZipAll {
        iter_left: left.into_iter(),
        iter_right: right.into_iter()
    }
}

/// Works like @zip_all, but inserts a default parameter instead of None in case if one of iterators depleted
/// __Returns:__ Iterator for pairs values
pub fn zip_all_with_default<'a, T, E>(left: T, right: T, default: E) -> Box<Iterator<Item = (E, E)>+ 'a> 
    where T: IntoIterator<Item=E> + 'a, E: Clone + 'a
{ 
    Box::new(zip_all(left, right).map(move |(l, r)| (l.unwrap_or(default.clone()), r.unwrap_or(default.clone()))))
}