extern crate rust_algorithms;

use rust_algorithms::datatypes::binary_heap::BinaryHeap;

#[test]
fn test_heap_push() {
    let mut tree = BinaryHeap::new();
    tree.push(1);
    println!("Tree {:?}", tree);
    tree.push(11);
    println!("Tree {:?}", tree);
    tree.push(51);
    println!("Tree {:?}", tree);
    tree.push(43);
    println!("Tree {:?}", tree);
    tree.push(19);
    println!("Tree {:?}", tree);
    tree.push(2);
    println!("Tree {:?}", tree);

    println!("Popping out {:?}", tree.pop());
    println!("Tree {:?}", tree);
    println!("Popping out {:?}", tree.pop());
    println!("Tree {:?}", tree);
    println!("Popping out {:?}", tree.pop());
    println!("Tree {:?}", tree);
    println!("Popping out {:?}", tree.pop());
    println!("Tree {:?}", tree);
    println!("Popping out {:?}", tree.pop());
    println!("Tree {:?}", tree);
    println!("Popping out {:?}", tree.pop());
    println!("Tree {:?}", tree);
    println!("Popping out {:?}", tree.pop());
    println!("Tree {:?}", tree);
}

