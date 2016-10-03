
/******************************************************************************
 * FISHER YATES'S SHUFFLE 
 *
 * Features:
 * 1. shuffle list in O(n) time.
 *
 * http://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
 * 
 ******************************************************************************/

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

// Vector implementation
impl<E> ArrayShuffle<E> for Vec<E> {}

// Slices implementation
impl<E> ArrayShuffle<E> for [E] {}

// Fixed-size implementation
// We use macro, to implent trait for all fixed-size arrays
implement_for_arrays!(ArrayShuffle);
