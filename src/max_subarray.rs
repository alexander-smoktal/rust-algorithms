/*******************************************************************************
 * MAXIMUM SUBARRAY
 *
 * In computer science, the maximum subarray problem is the task of finding the 
 * contiguous subarray within a one-dimensional array of numbers (containing at
 * least one positive number) which has the largest sum. For example, for the 
 * sequence of values −2, 1, −3, 4, −1, 2, 1, −5, 4; the contiguous subarray 
 * with the largest sum is 4, −1, 2, 1, with sum 6.
 *
 * The problem was first posed by Ulf Grenander of Brown University in 1977, as 
 * a simplified model for maximum likelihood estimation of patterns in digitized 
 * images. A linear time algorithm was found soon afterwards by Jay Kadane of 
 * Carnegie-Mellon University (Bentley 1984).
 *
 * http://en.wikipedia.org/wiki/Maximum_subarray_problem
 ******************************************************************************/
extern crate num;

use std::borrow::BorrowMut;

pub trait MaxSubarray<E> where Self: BorrowMut<[E]>, E: num::PrimInt {
    fn max_subarray(&self) -> &[E] {
        if self.borrow().is_empty() { return self.borrow() }
        let sums = self.borrow().iter().enumerate()                                // Enumarate to get all indices
                    .scan(E::zero(), |sum, (i, x)| { *sum = *sum + *x; Some((i, *sum)) });  // Calculate sum foreach element

        // Calculate pair of indices:
        // - first is start of subarray: first element with nonnegative prefix
        // - second is the end of subbrarray: maximum subbarray end starting from first
        let result = sums.fold(((0, E::zero() - E::one()), (0, E::zero() - E::one())),
                                |((si, sv), (ei, ev)), (index, sum)| {
                                    (
                                        if sv < E::zero() && sum >= E::zero() { (index, sum) } else { (si, sv) },
                                        if ev < sum { (index, sum) } else { (ei, ev) }
                                    )
                                }
             );

         let ((si, _), (ei, _)) = result;
         &self.borrow()[si..ei + 1]
     }
}

impl<E> MaxSubarray<E> for Vec<E> where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 0] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 1] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 2] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 3] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 4] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 5] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 6] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 7] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 8] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 9] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 10] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 11] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 12] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 13] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 14] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 15] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 16] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 17] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 18] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 19] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 20] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 21] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 22] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 23] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 24] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 25] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 26] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 27] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 28] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 29] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 30] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 31] where E: num::PrimInt {}
impl<E> MaxSubarray<E> for [E; 32] where E: num::PrimInt {}
