
use std::borrow::BorrowMut;
use std::clone::Clone;
extern crate rand;

pub fn shuffle<T, E>(array: &T) -> T where T: BorrowMut<[E]> + Clone  {
    let (mut array_clone, len) = (array.clone(), array.borrow().len());

    for i in 0..len {
        let j = rand::random::<usize>() % len;
        array_clone.borrow_mut().swap(i, j)
    }

    array_clone
}
