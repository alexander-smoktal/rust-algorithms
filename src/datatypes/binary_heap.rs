/******************************************************************************
 * BINARY TREE IMPLEMENTATION
 *
 * https://en.wikipedia.org/wiki/Binary_heap
 * 
 ******************************************************************************/

use super::tree::Tree;
use std::{ mem, cmp };
use std::ops::DerefMut;
use std::fmt::{Debug, Formatter, Error};

pub struct BinaryHeap<T> {
    tree: Tree<T>,
    cmp_func: Box<Fn(&T, &T) -> bool>
}

impl<T: Debug> Debug for BinaryHeap<T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        Debug::fmt(&self.tree, formatter)
    }
}

impl<T: Ord + Debug> BinaryHeap<T> {

    pub fn new_with_compare(cmp_func: Box<Fn(&T, &T) -> bool>) -> Self {
        BinaryHeap {
            tree: Tree::Empty,
            cmp_func: cmp_func
        }
    }

    pub fn new() -> Self {
        BinaryHeap {
            tree: Tree::Empty,
            cmp_func: Box::new(|x: &T, y: &T| { x > y })
        }
    }

    fn heapify(tree: &mut Tree<T>, cmp: &Box<Fn(&T, &T) -> bool>) {
        match tree {
            &mut Tree::Empty | &mut Tree::Node(_, 1, _, _) => (),
            &mut Tree::Node(ref mut self_element, _, ref mut left, ref mut right) => {
                if let &mut Tree::Node(ref mut lval, _, _, _) = left.deref_mut() {
                    if cmp(lval, self_element) {
                        mem::swap(self_element, lval);
                    }
                }
                if let &mut Tree::Node(ref mut rval, _, _, _) = right.deref_mut() {
                    if cmp(rval, self_element) {
                        mem::swap(self_element, rval);

                    }
                }
                Self::heapify(left, cmp);
                Self::heapify(right, cmp); 
            }
        };
    }

    // Put an element onto the heap
    pub fn push(&mut self, value: T) -> &Self {
        let tree = mem::replace(&mut self.tree, Tree::Empty);
        self.tree = match tree {
            Tree::Empty => Tree::leaf(value),
            Tree::Node(self_value, _, left, right) => {
                if left.size() < right.size() { 
                    Tree::node(value, Tree::node(self_value, *left, Tree::Empty), *right)
                } else {
                    Tree::node(value, *left, Tree::node(self_value, *right, Tree::Empty))
                }
            }
        };
        Self::heapify(&mut self.tree, &self.cmp_func);
        self
    }

    /// Get the largest element out of the heap
    fn internal_pop(tree: &mut Tree<T>, cmp: &Box<Fn(&T, &T) -> bool>) -> Option<T> {
        let result: Option<T>;
        let tree_copy = mem::replace(tree, Tree::Empty);
        *tree = match tree_copy {
            Tree::Empty => {
                result = None;
                Tree::Empty
            }
            Tree::Node(self_value, _, mut left, mut right) => {
                result = Some(self_value);
                if left.is_empty() && right.is_empty() {
                    Tree::Empty
                } else {
                    if left.size() < right.size() {
                        Tree::Node(Self::internal_pop(right.deref_mut(), cmp).unwrap(), cmp::max(left.size(), right.size()) + 1, left, right)
                    } else {
                        Tree::Node(Self::internal_pop(left.deref_mut(), cmp).unwrap(), cmp::max(left.size(), right.size()) + 1, left, right)
                    }
                }
            }
        };
        Self::heapify(tree, cmp);
        result
    }

    pub fn pop(&mut self) -> Option<T> {
        Self::internal_pop(&mut self.tree, &self.cmp_func)
    }
}