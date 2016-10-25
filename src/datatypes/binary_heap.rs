/******************************************************************************
 * BINARY TREE IMPLEMENTATION
 *
 * https://en.wikipedia.org/wiki/Binary_heap
 * 
 ******************************************************************************/

use super::tree::Tree;
use std::{ mem, cmp };
use std::ops::DerefMut;
use std::fmt::Debug;

pub type BinaryHeap<T> = Tree<T>;

impl<T: Ord + Debug> BinaryHeap<T> {
    pub fn new() -> Self {
        Tree::Empty
    }

    fn heapify(&mut self) -> &Self {
        match self {
            &mut Tree::Empty | &mut Tree::Node(_, 1, _, _) => (),
            &mut Tree::Node(ref mut self_element, _, ref mut left, ref mut right) => {
                if let &mut Tree::Node(ref mut lval, _, _, _) = left.deref_mut() {
                    if lval > self_element {
                        mem::swap(self_element, lval);
                    }
                }
                if let &mut Tree::Node(ref mut rval, _, _, _) = right.deref_mut() {
                    if rval > self_element {
                        mem::swap(self_element, rval);

                    }
                }
                left.heapify();
                right.heapify();
            }
        };
        self
    }

    // Put an element onto the heap
    pub fn push(&mut self, value: T) -> &Self {
        let tree = mem::replace(self, Tree::Empty);
        *self = match tree {
            Tree::Empty => Tree::leaf(value),
            Tree::Node(self_value, _, left, right) => {
                if left.size() < right.size() { Tree::node(value, Tree::node(self_value, *left, Tree::Empty), *right) }
                else { Tree::node(value, *left, Tree::node(self_value, *right, Tree::Empty)) }
            }
        };
        self.heapify()
    }

    /// Get the largest element out of the heap
    pub fn pop(&mut self) -> Option<T> {
        let result: Option<T>;
        let tree = mem::replace(self, Tree::Empty);
        *self = match tree {
            Tree::Empty => {
                result = None;
                Tree::Empty
            }
            Tree::Node(self_value, _, mut left, mut right) => {
                result = Some(self_value);
                if left.is_empty() && right.is_empty() {
                    Tree::Empty
                } else {
                    if left.size() < right.size() { Tree::Node((right.deref_mut() as &mut Self).pop().unwrap(), cmp::max(left.size(), right.size()) + 1, left, right) }
                    else { Tree::Node((left.deref_mut() as &mut Self).pop().unwrap(), cmp::max(left.size(), right.size()) + 1, left, right) }
                }
            }
        };
        self.heapify();
        result
    }
}