
use std::borrow::BorrowMut;

extern crate rand;

pub trait ArrayShuffle<E> where Self: BorrowMut<[E]> {
     fn shuffle(&mut self) {
         let len = self.borrow().len();

         for i in 0..len {
            let j = rand::random::<usize>() % (len - i);
            self.borrow_mut().swap(i, i + j)
        }
     }
}

// Shuffle trait for vector
impl<E> ArrayShuffle<E> for Vec<E> {}

// Shuffle trait for slices
impl<E> ArrayShuffle<E> for [E] {}

// Shuffle trait for fixed-size arrays.
// We use macro, to implent trait for all fixed-size arrays
implement_for_arrays!(ArrayShuffle);
