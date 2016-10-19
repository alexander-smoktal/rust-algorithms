/*******************************************************************************
 * QUEUE
 *
 * http://en.wikipedia.org/wiki/Queue_(data_structure)
 *
 ******************************************************************************/

use std::rc::Rc;
use std::fmt::{Display, Formatter, Result};
use std::cell::RefCell;

const QUEUE_VEC_LEN: usize = 2;

struct QueueNode<T> {
    data: Vec<T>,
    next: Option<Rc<RefCell<QueueNode<T>>>>
}

pub struct Queue<T> {
    first_node: Rc<RefCell<QueueNode<T>>>,
    last_node: Rc<RefCell<QueueNode<T>>>
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        let root_node = Rc::new(
            RefCell::new (
                QueueNode {
                    data: Vec::with_capacity(QUEUE_VEC_LEN),
                    next: None
                }
            )
        );

        Queue {
            first_node: root_node.clone(),
            last_node: root_node
        }
    }

    pub fn push(&mut self, elem: T) -> &mut Self {
        {
            if self.last_node.borrow().data.len() >= QUEUE_VEC_LEN {
                self.last_node.borrow_mut().next = Some (
                    Rc::new(
                        RefCell::new(
                            QueueNode {
                                data: Vec::with_capacity(QUEUE_VEC_LEN),
                                next: None
                            }
                        )
                    )
                );

                let next_last_node = self.last_node.borrow().next.as_ref().unwrap().clone();
                self.last_node = next_last_node;
            }
        }

        self.last_node.borrow_mut().data.push(elem);
        self
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.first_node.borrow().data.is_empty() {
            if self.first_node.borrow().next.is_none() { return None }

            let next_first_node = self.first_node.borrow().next.as_ref().unwrap().clone();
            self.first_node = next_first_node;
        }

        Some(self.first_node.borrow_mut().data.remove(0))
    }
}


impl<T> Display for Queue<T> where T: Display {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let _ = write!(f, "{{");
        let mut initial = true;

        let mut pointer = self.first_node.clone();
        loop {
            for elem in &pointer.borrow().data {
                // This comma -_-
                if initial { initial = false } else { let _ = write!(f, ", "); }

                let _ = write!(f, "{}", elem);
            }

            if pointer.borrow().next.is_none() { break }

            let next_pointer = pointer.borrow().next.as_ref().unwrap().clone();
            pointer = next_pointer
        }
        write!(f, "}}")
    }
}
