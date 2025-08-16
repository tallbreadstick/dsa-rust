use crate::{adt::{Queue, Stack}, linked_list::{doubly, singly}};

impl<T: Clone> Stack<T> for singly::LinkedList<T> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn push(&mut self, data: T) {
        self.push_head(data);
    }
    fn pop(&mut self) -> Option<T> {
        self.pop()
    }
    fn peek(&self) -> Option<T> {
        if let Some(head) = &self.head {
            Some(head.data.clone())
        } else {
            None
        }
    }
    fn clear(&mut self) {
        self.clear();
    }
}

impl<T: Clone> Queue<T> for singly::LinkedList<T> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn offer(&mut self, data: T) {
        self.push_tail(data);
    }
    fn poll(&mut self) -> Option<T> {
        self.pop()
    }
    fn head(&self) -> Option<T> {
        if let Some(head) = &self.head {
            Some(head.data.clone())
        } else {
            None
        }
    }
    fn tail(&self) -> Option<T> {
        if !self.tail.is_null() {
            unsafe {
                Some(Box::from_raw(self.tail).data.clone())
            }
        } else {
            None
        }
    }
    fn clear(&mut self) {
        self.clear();
    }
}

impl<T: Clone> Stack<T> for doubly::LinkedList<T> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn push(&mut self, data: T) {
        self.push_head(data);
    }
    fn pop(&mut self) -> Option<T> {
        self.pop_head()
    }
    fn peek(&self) -> Option<T> {
        self.get(0)
    }
    fn clear(&mut self) {
        self.clear();
    }
}

impl<T: Clone> Queue<T> for doubly::LinkedList<T> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn offer(&mut self, data: T) {
        self.push_tail(data);
    }
    fn poll(&mut self) -> Option<T> {
        self.pop_head()
    }
    fn head(&self) -> Option<T> {
        if let Some(head) = self.head.clone() {
            Some(head.borrow().data.clone())
        } else {
            None
        }
    }
    fn tail(&self) -> Option<T> {
        if let Some(tail) = self.tail.clone() {
            Some(tail.borrow().data.clone())
        } else {
            None
        }
    }
    fn clear(&mut self) {
        self.clear();
    }
}