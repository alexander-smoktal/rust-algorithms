/*******************************************************************************
 * QUEUE
 *
 * http://en.wikipedia.org/wiki/Queue_(data_structure)
 *
 ******************************************************************************/

extern crate alloc;

use self::alloc::heap;
use std::mem;
use std::fmt;

const QUEUE_VEC_LEN: usize = 2;

struct QueueNode<T> {
    pub data: Vec<T>,
    pub next: *mut QueueNode<T>
}

pub struct Queue<T> {
    first_node: *mut QueueNode<T>,
    last_node: *mut QueueNode<T>
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        unsafe {
            let new_node = heap::allocate(mem::size_of::<QueueNode<T>>(),
                                        mem::align_of::<QueueNode<T>>())
                        as *mut QueueNode<T>;

            let new_node_ref = &mut *new_node;
            new_node_ref.data = Vec::with_capacity(QUEUE_VEC_LEN);
            new_node_ref.next = heap::EMPTY as *mut QueueNode<T>;
            
            Queue {
                first_node: new_node,
                last_node: new_node
            }
        }
    }

    pub fn push(&mut self, elem: T) -> &mut Self {
        unsafe{
            let mut last_node = &mut *self.last_node;
            if last_node.data.len() >= QUEUE_VEC_LEN {
                last_node.next = heap::allocate(mem::size_of::<QueueNode<T>>(),
                                                mem::align_of::<QueueNode<T>>())
                                 as *mut QueueNode<T>;

                self.last_node = last_node.next;
                last_node = &mut *self.last_node;
                last_node.data = Vec::with_capacity(QUEUE_VEC_LEN);
                last_node.next = heap::EMPTY as *mut QueueNode<T>;
            }
            last_node.data.push(elem);
        }
        self
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            let first_node = &mut *self.first_node; 

            if first_node.data.is_empty() {
                if first_node.next == heap::EMPTY as *mut QueueNode<T> {
                    return None
                }

                let second_node = first_node.next;
                heap::deallocate(self.first_node as *mut u8,
                                 mem::size_of::<QueueNode<T>>(),
                                 mem::align_of::<QueueNode<T>>());
                self.first_node = second_node;
            }
            let first_node = &mut *self.first_node; 

            Some(first_node.data.remove(0)) 
        }
    }
}


impl<T> fmt::Display for Queue<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "{{");
        let mut initial = true;
        unsafe {
            let mut pointer = &mut *self.first_node;
            loop {
                for elem in &pointer.data {
                    // This comma -_-
                    if initial { initial = false } else { let _ = write!(f, ", "); }

                    let _ = write!(f, "{}", elem);
                }
                if pointer.next == heap::EMPTY as *mut QueueNode<T> { break }
                pointer = &mut *pointer.next
            }
        }
        write!(f, "}}")
    }
}
