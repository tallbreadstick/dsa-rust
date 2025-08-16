pub mod linked_list;

pub trait Stack<T: Clone> {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn push(&mut self, data: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<T>;
    fn clear(&mut self);
}

pub trait Queue<T: Clone> {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn offer(&mut self, data: T);
    fn poll(&mut self) -> Option<T>;
    fn head(&self) -> Option<T>;
    fn tail(&self) -> Option<T>;
    fn clear(&mut self);
}