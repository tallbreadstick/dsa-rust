/*

    SINGLY-LINKED LIST

    implemented via Box pointer
    with raw mut ptr as tail

*/

use std::{fmt::Debug, ptr::null_mut};

pub type NodePtr<T> = Option<Box<Node<T>>>;

pub struct Node<T: Clone> {
    pub data: T,
    pub next: NodePtr<T>,
}

impl<T: Clone> Node<T> {
    pub fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

impl<T: Clone> Into<NodePtr<T>> for Node<T> {
    fn into(self) -> NodePtr<T> {
        Some(Box::new(self))
    }
}

pub struct LinkedList<T: Clone> {
    pub head: NodePtr<T>,
    pub tail: *mut Node<T>,
    pub size: usize,
}

#[macro_export]
macro_rules! singly_linked {
    [ $val:expr ; $count:expr ] => {{
        let mut list = $crate::linked_list::singly::LinkedList::new();
        for _ in 0..$count {
            list.push_head($val.clone());
        }
        list
    }};
    [ $( $x:expr ),* $(,)? ] => {
        $crate::linked_list::singly::LinkedList::from([ $( $x ),* ])
    };
}

impl<T: Clone> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: null_mut(),
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
        node.next = self.head.take();
        self.head = node.into();
        if self.tail.is_null() {
            self.tail = self
                .head
                .as_mut()
                .map(|ptr| ptr.as_mut() as *mut Node<T>)
                .unwrap();
        }
        self.size += 1;
    }
    pub fn push_tail(&mut self, data: T) {
        let mut new_node = Box::new(Node::new(data));
        let raw_node: *mut Node<T> = &mut *new_node;
        if let Some(tail_ptr) = (!self.tail.is_null()).then_some(self.tail) {
            unsafe {
                (*tail_ptr).next = Some(new_node);
            }
        } else {
            self.head = Some(new_node);
        }
        self.tail = raw_node;
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
    pub fn pop(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            self.head = head.next;
            self.size -= 1;
            if self.is_empty() {
                self.tail = null_mut();
            }
            Some(head.data)
        } else {
            None
        }
    }
    pub fn insert(&mut self, index: usize, data: T) -> Option<()> {
        if index < self.size {
            if index == 0 {
                self.push_head(data);
            } else {
                let mut i = 0;
                let mut ptr = &mut self.head;
                while let Some(node) = ptr {
                    if i == index {
                        let mut new = Node::new(data);
                        new.next = node.next.take();
                        node.next = new.into();
                        break;
                    }
                    i += 1;
                    ptr = &mut node.next;
                }
            }
            Some(())
        } else {
            None
        }
    }
    pub fn set(&mut self, index: usize, data: T) -> Option<()> {
        if index < self.size {
            let mut i = 0;
            let mut ptr = &mut self.head;
            while let Some(node) = ptr {
                if i == index {
                    node.data = data;
                    break;
                }
                i += 1;
                ptr = &mut node.next;
            }
            Some(())
        } else {
            None
        }
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.size {
            let mut i = 0;
            let mut ptr = &self.head;
            while let Some(node) = ptr {
                if i == index {
                    return Some(&node.data);
                }
                i += 1;
                ptr = &node.next;
            }
        }
        None
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.size {
            let mut i = 0;
            let mut ptr = &mut self.head;
            while let Some(node) = ptr {
                if i == index {
                    return Some(&mut node.data);
                }
                i += 1;
                ptr = &mut node.next;
            }
        }
        None
    }
    pub fn clear(&mut self) {
        self.head = None;
        self.tail = null_mut();
        self.size = 0;
    }
}

impl<T: Clone + Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut ptr = &self.head;
        while let Some(node) = ptr {
            write!(f, "{:?}", node.data)?;
            if node.next.is_some() {
                write!(f, ", ")?;
            }
            ptr = &node.next;
        }
        write!(f, "]")
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut list = Self::new();
        self.iter().for_each(|item| list.push_tail(item.clone()));
        list
    }
}

pub struct Iter<'a, T: Clone> {
    current: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T: Clone> {
    current: Option<&'a mut Node<T>>,
}

pub struct IntoIter<T: Clone> {
    collection: LinkedList<T>,
}

impl<'a, T: Clone> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.data
        })
    }
}

impl<'a, T: Clone> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        let current_node = self.current.take()?;
        let (data, next) = {
            let Node { data, next } = current_node;
            (data, next.as_deref_mut())
        };
        self.current = next;
        Some(data)
    }
}

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.collection.pop()
    }
}

impl<T: Clone> LinkedList<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            current: self.head.as_deref(),
        }
    }
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            current: self.head.as_deref_mut(),
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