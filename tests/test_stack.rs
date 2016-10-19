extern crate rust_algorithms;

use rust_algorithms::datatypes::stack::Stack;

#[test]
fn test_stack_empty() {
    let mut stack = Stack::<u64>::new();

    println!("Stack: {}", stack);
    assert_eq!(stack.pop(), None);
}

#[test]
fn test_stack_constructor() {
    let mut stack = Stack::<u64>::new();

    stack.push(5).push(6).push(8).push(10).push(95);

    println!("Stack: {}", stack);

    assert_eq!(stack.pop(), Some(95));
    assert_eq!(stack.pop(), Some(10));
    assert_eq!(stack.pop(), Some(8));
    assert_eq!(stack.pop(), Some(6));
    assert_eq!(stack.pop(), Some(5));
    assert_eq!(stack.pop(), None);

    stack.push(67).push(1).push(0);

    println!("Stack: {}", stack);

    assert_eq!(stack.pop(), Some(0));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), Some(67));
    assert_eq!(stack.pop(), None);
}