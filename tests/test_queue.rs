extern crate rust_algorithms;

use rust_algorithms::datatypes::queue::Queue;

#[test]
fn test_queue_empty() {
    let mut queue = Queue::<u64>::new();

    println!("Queue: {}", queue);
    assert_eq!(queue.pop(), None);
}

#[test]
fn test_queue_constructor() {
    let mut queue = Queue::<u64>::new();

    queue.push(5).push(6).push(8).push(10).push(95);

    println!("Queue: {}", queue);

    assert_eq!(queue.pop(), Some(5));
    assert_eq!(queue.pop(), Some(6));
    assert_eq!(queue.pop(), Some(8));
    assert_eq!(queue.pop(), Some(10));
    assert_eq!(queue.pop(), Some(95));
    assert_eq!(queue.pop(), None);

    queue.push(67).push(1).push(0);

    println!("Queue: {}", queue);

    assert_eq!(queue.pop(), Some(67));
    assert_eq!(queue.pop(), Some(1));
    assert_eq!(queue.pop(), Some(0));
    assert_eq!(queue.pop(), None);
}