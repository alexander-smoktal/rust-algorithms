/******************************************************************************
 * BINARY TREE IMPLEMENTATION
 *
 * https://en.wikipedia.org/wiki/Tree_(data_structure)
 * 
 ******************************************************************************/

use std::cmp;

#[derive(Debug)]
pub enum Tree<T> {
    // Element, height, left, right
    Node (T, usize, Box<Tree<T>>, Box<Tree<T>>),
    Empty
}

impl<T> Tree<T> {
    pub fn leaf(value: T) -> Self {
        Tree::Node(value, 1, Box::new(Tree::Empty), Box::new(Tree::Empty))
    }

    pub fn node(value: T, left: Tree<T>, right: Tree<T>) -> Self {
        let max_size = cmp::max(left.size(), right.size());
        Tree::Node(value, max_size + 1, Box::new(left), Box::new(right))
    }

    pub fn size(&self) -> usize {
        match self {
            &Tree::Empty => 0,
            &Tree::Node(_, size, _, _) => size
        }
    }

    pub fn value(&self) -> &T {
        match self {
            &Tree::Empty => panic!("Getting elment of empty tree"),
            &Tree::Node(ref e, _, _, _) => e
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            &Tree::Empty => true,
            _ => false
        }
    }
}