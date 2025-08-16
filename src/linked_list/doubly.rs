/*

    DOUBLY-LINKED LIST

    Implemented via rc-refcell pointers

*/

use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Node<T: Clone> {
    pub data: T,
    pub next: NodePtr<T>,
    pub prev: NodePtr<T>,
}

impl<T: Clone> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            next: None,
            prev: None,
        }
    }
}

impl<T: Clone> Into<NodePtr<T>> for Node<T> {
    fn into(self) -> NodePtr<T> {
        Some(Rc::new(RefCell::new(self)))
    }
}

pub struct LinkedList<T: Clone> {
    pub head: NodePtr<T>,
    pub tail: NodePtr<T>,
    pub size: usize,
}

#[macro_export]
macro_rules! doubly_linked {
    [ $val:expr ; $count:expr ] => {{
        let mut list = $crate::linked_list::doubly::LinkedList::new();
        for _ in 0..$count {
            list.push_head($val.clone());
        }
        list
    }};
    [ $( $x:expr ),* $(,)? ] => {
        $crate::linked_list::doubly::LinkedList::from([ $( $x ),* ])
    };
}

impl<T: Clone> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }
    pub fn from<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut list = Self::new();
        list.extend_tail(iter);
        list
    }
}

impl<T: Clone> LinkedList<T> {
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn push_head(&mut self, data: T) {
        let mut node = Node::new(data);
        node.next = self.head.clone();
        let ptr: NodePtr<T> = node.into();
        if let Some(head) = self.head.clone() {
            head.borrow_mut().prev = ptr.clone();
        } else {
            self.tail = ptr.clone();
        }
        self.head = ptr;
        self.size += 1;
    }
    pub fn push_tail(&mut self, data: T) {
        let mut node = Node::new(data);
        node.prev = self.tail.clone();
        let ptr: NodePtr<T> = node.into();
        if let Some(tail) = self.tail.clone() {
            tail.borrow_mut().next = ptr.clone();
        } else {
            self.head = ptr.clone();
        }
        self.tail = ptr;
        self.size += 1;
    }
    pub fn extend_head<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for i in iter {
            self.push_head(i);
        }
    }
    pub fn extend_tail<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for i in iter {
            self.push_tail(i);
        }
    }
    pub fn pop_head(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            if let Some(next) = head.borrow().next.clone() {
                next.borrow_mut().prev = None;
            }
            self.head = head.borrow().next.clone();
            Rc::try_unwrap(head).ok().map(|node| {
                self.size -= 1;
                node.into_inner().data
            })
        } else {
            None
        }
    }
    pub fn pop_tail(&mut self) -> Option<T> {
        if let Some(tail) = self.tail.take() {
            if let Some(prev) = tail.borrow().prev.clone() {
                prev.borrow_mut().next = None;
            }
            self.tail = tail.borrow().prev.clone();
            Rc::try_unwrap(tail).ok().map(|node| {
                self.size -= 1;
                node.into_inner().data
            })
        } else {
            None
        }
    }
    pub fn insert(&mut self, index: usize, data: T) -> Option<()> {
        if index < self.size {
            let mut i = 0;
            let mut ptr = self.head.clone();
            while let Some(node) = ptr.clone() {
                if i == index {
                    break;
                }
                i += 1;
                ptr = node.borrow().next.clone();
            }
            if let Some(node) = ptr.clone() {
                let mut new = Node::new(data);
                new.next = node.borrow().next.clone();
                new.prev = ptr;
                let node_ptr: NodePtr<T> = new.into();
                if let Some(next) = node.borrow().next.clone() {
                    next.borrow_mut().prev = node_ptr.clone();
                }
                node.borrow_mut().next = node_ptr;
            }
            Some(())
        } else {
            None
        }
    }
    pub fn set(&mut self, index: usize, data: T) -> Option<()> {
        if index < self.size {
            let mut i = 0;
            let mut ptr = self.head.clone();
            while let Some(node) = ptr {
                if i == index {
                    node.borrow_mut().data = data;
                    break;
                }
                i += 1;
                ptr = node.borrow().next.clone();
            }
            Some(())
        } else {
            None
        }
    }
    pub fn get(&self, index: usize) -> Option<T> {
        if index < self.size {
            let mut i = 0;
            let mut ptr = self.head.clone();
            while let Some(node) = ptr {
                if i == index {
                    return Some(node.borrow().data.clone());
                }
                i += 1;
                ptr = node.borrow().next.clone();
            }
        }
        None
    }
    pub fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.size = 0;
    }
}

impl<T: Clone + Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut ptr = self.head.clone();
        while let Some(node) = ptr {
            write!(f, "{:?}", node.borrow().data)?;
            if node.borrow().next.is_some() {
                write!(f, ", ")?;
            }
            ptr = node.borrow().next.clone();
        }
        write!(f, "]")
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut list = Self::new();
        self.iter().for_each(|item| list.push_tail(item));
        list
    }
}

pub struct Iter<T: Clone> {
    current: NodePtr<T>,
}

pub struct IntoIter<T: Clone> {
    collection: LinkedList<T>,
}

impl<T: Clone> Iterator for Iter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.current.clone() {
            self.current = node.borrow().next.clone();
            Some(node.borrow().data.clone())
        } else {
            None
        }
    }
}

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.collection.pop_head()
    }
}

impl<T: Clone> LinkedList<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            current: self.head.clone(),
        }
    }
}

impl<T: Clone> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { collection: self }
    }
}

impl<T: Clone> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from(iter)
    }
}