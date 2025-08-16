use std::collections::LinkedList;

use crate::tree::{avl::AVLTree, heap::BinaryHeap};

pub mod adt;
pub mod tree;
pub mod linked_list;

fn main() {

    let mut heap = BinaryHeap::from(|a, b| a.cmp(b), (0..10).rev());

    println!("{:?}", heap);
    while let Some(num) = heap.poll() {
        println!("{}", num);
    }

    heap.extend((0..5).rev());

    println!("{:?}", heap);
    for num in heap.iter_unsorted() {
        println!("{}", num);
    }

}
