use crate::{linked_list::doubly::LinkedList, tree::heap::BinaryHeap};

pub mod adt;
pub mod linked_list;
pub mod tree;

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

    let list = LinkedList::from(0..10);

    println!("{:?}", list);

    for i in list.iter() {
        i.borrow_mut().data *= 7;
    }

    println!("{:?}", list);
}
