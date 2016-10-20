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
use lc_generator::Random;

pub trait ArrayShuffle<E> where Self: BorrowMut<[E]> {
     fn shuffle(&mut self) {
         let len = self.borrow().len();

         for i in 0..len - 1 {
            let j = usize::random() % (len - i - 1) + 1;
            self.borrow_mut().swap(i, i + j)
        }
     }
}

impl<E> ArrayShuffle<E> for Vec<E> {}
impl<E> ArrayShuffle<E> for [E] {}
impl<E> ArrayShuffle<E> for [E; 0] {}
impl<E> ArrayShuffle<E> for [E; 1] {}
impl<E> ArrayShuffle<E> for [E; 2] {}
impl<E> ArrayShuffle<E> for [E; 3] {}
impl<E> ArrayShuffle<E> for [E; 4] {}
impl<E> ArrayShuffle<E> for [E; 5] {}
impl<E> ArrayShuffle<E> for [E; 6] {}
impl<E> ArrayShuffle<E> for [E; 7] {}
impl<E> ArrayShuffle<E> for [E; 8] {}
impl<E> ArrayShuffle<E> for [E; 9] {}
impl<E> ArrayShuffle<E> for [E; 10] {}
impl<E> ArrayShuffle<E> for [E; 11] {}
impl<E> ArrayShuffle<E> for [E; 12] {}
impl<E> ArrayShuffle<E> for [E; 13] {}
impl<E> ArrayShuffle<E> for [E; 14] {}
impl<E> ArrayShuffle<E> for [E; 15] {}
impl<E> ArrayShuffle<E> for [E; 16] {}
impl<E> ArrayShuffle<E> for [E; 17] {}
impl<E> ArrayShuffle<E> for [E; 18] {}
impl<E> ArrayShuffle<E> for [E; 19] {}
impl<E> ArrayShuffle<E> for [E; 20] {}
impl<E> ArrayShuffle<E> for [E; 21] {}
impl<E> ArrayShuffle<E> for [E; 22] {}
impl<E> ArrayShuffle<E> for [E; 23] {}
impl<E> ArrayShuffle<E> for [E; 24] {}
impl<E> ArrayShuffle<E> for [E; 25] {}
impl<E> ArrayShuffle<E> for [E; 26] {}
impl<E> ArrayShuffle<E> for [E; 27] {}
impl<E> ArrayShuffle<E> for [E; 28] {}
impl<E> ArrayShuffle<E> for [E; 29] {}
impl<E> ArrayShuffle<E> for [E; 30] {}
impl<E> ArrayShuffle<E> for [E; 31] {}
impl<E> ArrayShuffle<E> for [E; 32] {}