use super::binary_heap::BinaryHeap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct FibonacciHeap<T> {
    set: Vec<BinaryHeap<T>>
}

impl<T: Debug + Ord> FibonacciHeap<T> {
    pub fn new() -> Self {
        FibonacciHeap {
            set: vec![]
        }
    }

    /// Creates minimum heap for internal usage
    fn create_binary_heap() -> BinaryHeap<T> {
        BinaryHeap::new_with_compare(Box::new(|x, y| { x < y }))
    }

    pub fn merge(mut self, mut other: Self) -> Self {
        self.set.append(&mut other.set);
        self
    }

    pub fn insert(self, element: T) -> Self {
        let mut heap = Self::create_binary_heap();
        heap.push(element);
        self.merge(FibonacciHeap {
            set: vec![heap]
        })
    }
}