use std::{cmp::Ordering, fmt::Debug};

fn left(i: usize) -> usize { 2 * i + 1 }
fn right(i: usize) -> usize { 2 * i + 2 }
fn parent(i: usize) -> usize { (i - 1) / 2 }

pub struct BinaryHeap<T>
where
    T: Clone + Ord + PartialOrd,
{
    heap: Vec<T>,
    cmp: fn(&T, &T) -> Ordering,
}

impl<T> BinaryHeap<T>
where
    T: Clone + Ord + PartialOrd,
{
    pub fn new(cmp: fn(&T, &T) -> Ordering) -> Self {
        Self { heap: Vec::new(), cmp }
    }
    pub fn min() -> Self {
        Self { heap: Vec::new(), cmp: |a, b| a.cmp(b) }
    }
    pub fn max() -> Self {
        Self { heap: Vec::new(), cmp: |a, b| b.cmp(a) }
    }
    pub fn from<I>(cmp: fn(&T, &T) -> Ordering, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut heap = Self::new(cmp);
        heap.extend(iter);
        heap
    }
}

impl<T> BinaryHeap<T>
where
    T: Clone + Ord + PartialOrd,
{
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
    pub fn size(&self) -> usize {
        self.heap.len()
    }
    pub fn offer(&mut self, data: T) {
        self.heap.push(data);
        let mut i = self.heap.len() - 1;
        while i > 0 {
            let p = parent(i);
            if (self.cmp)(&self.heap[i], &self.heap[p]).is_lt() {
                self.heap.swap(i, p);
                i = p;
            } else {
                break;
            }
        }
    }
    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in iter {
            self.offer(item);
        }
    }
    pub fn poll(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }
        let val = self.heap.swap_remove(0);
        if !self.heap.is_empty() {
            let mut i = 0;
            loop {
                let l = left(i);
                if l >= self.heap.len() {
                    break;
                }
                let r = right(i);
                let best = if r < self.heap.len() &&
                    (self.cmp)(&self.heap[r], &self.heap[l]).is_lt()
                {
                    r
                } else {
                    l
                };
                if (self.cmp)(&self.heap[best], &self.heap[i]).is_lt() {
                    self.heap.swap(i, best);
                    i = best;
                } else {
                    break;
                }
            }
        }
        Some(val)
    }
    pub fn peek(&self) -> Option<&T> {
        self.heap.get(0)
    }
    pub fn clear(&mut self) {
        self.heap.clear();
    }
    pub fn shrink_to_fit(&mut self) {
        self.heap.shrink_to_fit();
    }
}

impl<T> Debug for BinaryHeap<T>
where
    T: Debug + Clone + Ord + PartialOrd,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.heap).finish()
    }
}

impl<T> Clone for BinaryHeap<T>
where
    T: Clone + Ord + PartialOrd,
{
    fn clone(&self) -> Self {
        Self { heap: self.heap.clone(), cmp: self.cmp }
    }
}

pub struct Iter<'a, T>
where 
    T: Clone + Ord + PartialOrd
{
    collection: &'a [T],
    index: usize
}

pub struct IntoIter<T>
where
    T: Clone + Ord + PartialOrd,
{
    consumer: BinaryHeap<T>,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Clone + Ord + PartialOrd
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.collection.get(self.index) {
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<'a, T> BinaryHeap<T>
where
    T: Clone + Ord + PartialOrd
{
    pub fn iter_unsorted(&'a self) -> Iter<'a, T> {
        Iter { collection: &self.heap, index: 0 }
    }
}

impl<T> Iterator for IntoIter<T>
where
    T: Clone + Ord + PartialOrd,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.consumer.poll()
    }
}

impl<T> IntoIterator for BinaryHeap<T>
where
    T: Clone + Ord + PartialOrd,
{
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { consumer: self }
    }
}
