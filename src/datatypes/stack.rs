/*******************************************************************************
 * STACK
 *
 * http://en.wikipedia.org/wiki/Stack_(data_structure)
 *
 ******************************************************************************/

use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;

const STACK_VEC_LEN: usize = 2;

struct StackNode<T> {
    data: Vec<T>,
    previous: Option<Rc<StackNode<T>>>
}

pub struct Stack<T> {
    node: Rc<StackNode<T>>
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            node: Rc::new (
                StackNode {
                    data: Vec::with_capacity(STACK_VEC_LEN),
                    previous: None
                }
            )
        }
    }

    pub fn push(&mut self, elem: T) -> &mut Self {
        {
            if self.node.data.len() >= STACK_VEC_LEN {
                self.node = Rc::new (
                    StackNode {
                        data: Vec::with_capacity(STACK_VEC_LEN),
                        previous: Some (self.node.clone())
                    }
                );
            }
        }

        Rc::get_mut(&mut self.node).unwrap().data.push(elem);
        self
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.node.data.is_empty() {
            if self.node.previous.is_none() { return None }

            let next_node = self.node.previous.as_ref().unwrap().clone();
            self.node = next_node
        }

        Some(Rc::get_mut(&mut self.node).unwrap().data.pop().unwrap())
    }
}


impl<T> Display for Stack<T> where T: Display {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let _ = write!(f, "{{");
        let mut initial = true;

        let mut pointer = &self.node;
        loop {
            for elem in &pointer.data {
                // This comma -_-
                if initial { initial = false } else { let _ = write!(f, ", "); }

                let _ = write!(f, "{}", elem);
            }

            if pointer.previous.is_none() { break }

            pointer = pointer.previous.as_ref().unwrap()
        }
        write!(f, "}}")
    }
}
