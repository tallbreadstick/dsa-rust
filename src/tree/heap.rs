use std::{cmp::Ordering, fmt::Debug};

fn left(i: usize) -> usize { 2 * i }
fn right(i: usize) -> usize { 2 * i + 1 }
fn parent(i: usize) -> usize { i / 2 }

pub struct BinaryHeap<T>
where
    T: Clone + Ord + PartialOrd,
{
    heap: Vec<Option<T>>,
    size: usize,
    cmp: fn(&T, &T) -> Ordering,
}

impl<T> BinaryHeap<T>
where
    T: Clone + Ord + PartialOrd,
{
    pub fn new(cmp: fn(&T, &T) -> Ordering) -> Self {
        Self {
            heap: vec![None],
            size: 0,
            cmp,
        }
    }
    pub fn min() -> Self {
        Self {
            heap: vec![None],
            size: 0,
            cmp: |a, b| a.cmp(b),
        }
    }
    pub fn max() -> Self {
        Self {
            heap: vec![None],
            size: 0,
            cmp: |a, b| b.cmp(a),
        }
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
        self.size == 0
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn offer(&mut self, data: T) {
        self.heap.push(Some(data));
        self.size += 1;
        let mut i = self.size;
        while i > 1 {
            let p = parent(i);
            match (&self.heap[i], &self.heap[p]) {
                (Some(current), Some(parent)) if (self.cmp)(current, parent).is_lt() => {
                    self.heap.swap(i, p);
                }
                _ => break,
            }
            i = p;
        }
    }
    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        iter.into_iter().for_each(|item| {
            self.offer(item);
        });
    }
    pub fn poll(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let val = self.heap[1].take();
        if self.size == 1 {
            self.heap.pop();
            self.size = 0;
            return val;
        }
        let last = self.heap.pop().unwrap();
        self.size -= 1;
        self.heap[1] = last;
        let mut i = 1;
        loop {
            let l = left(i);
            if l > self.size {
                break;
            }
            let r = right(i);
            let best = if r <= self.size {
                let left_ref = self.heap[l].as_ref().unwrap();
                let right_ref = self.heap[r].as_ref().unwrap();
                if (self.cmp)(left_ref, right_ref).is_le() {
                    l
                } else {
                    r
                }
            } else {
                l
            };
            let child = self.heap[best].as_ref().unwrap();
            let curr = self.heap[i].as_ref().unwrap();
            if (self.cmp)(child, curr).is_lt() {
                self.heap.swap(i, best);
                i = best;
            } else {
                break;
            }
        }
        val
    }
    pub fn peek(&self) -> Option<&T> {
        if let Some(root) = self.heap.get(1) {
            root.as_ref()
        } else {
            None
        }
    }
    pub fn clear(&mut self) {
        self.heap.truncate(1);
        self.size = 0;
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
        write!(f, "[")?;
        for i in 1..=self.size {
            if let Some(item) = &self.heap[i] {
                write!(f, "{:?}", item)?;
                if i < self.size {
                    write!(f, ", ")?;
                }
            }
        }
        write!(f, "]")
    }
}

impl<T> Clone for BinaryHeap<T>
where
    T: Debug + Clone + Ord + PartialOrd,
{
    fn clone(&self) -> Self {
        Self {
            heap: self.heap.clone(),
            size: self.size,
            cmp: self.cmp,
        }
    }
}

pub struct Iter<'a, T>
where 
    T: Clone + Ord + PartialOrd
{
    collection: &'a Vec<Option<T>>,
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
            item.as_ref()
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
        Iter { collection: &self.heap, index: 1 }
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
