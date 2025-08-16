use std::fmt::Debug;

use crate::linked_list::singly::LinkedList;

pub type NodePtr<T> = Option<Box<Node<T>>>;

pub struct Node<T>
where
    T: Clone + Ord + PartialOrd + Eq,
{
    pub data: T,
    pub left: NodePtr<T>,
    pub right: NodePtr<T>,
    pub height: i8,
}

impl<T> Into<NodePtr<T>> for Node<T>
where
    T: Clone + Ord + PartialOrd + Eq,
{
    fn into(self) -> NodePtr<T> {
        Some(Box::new(self))
    }
}

fn height<T>(ptr: &NodePtr<T>) -> i8
where
    T: Clone + Ord + PartialOrd + Eq,
{
    if let Some(node) = ptr { node.height } else { 0 }
}

fn insert_rec<T>(ptr: NodePtr<T>, data: &T) -> (NodePtr<T>, bool)
where
    T: Clone + Ord + PartialOrd + Eq,
{
    if let Some(mut node) = ptr {
        let inserted: bool;
        if data < &node.data {
            let (new_left, ok) = insert_rec(node.left, data);
            node.left = new_left;
            inserted = ok;
        } else if data > &node.data {
            let (new_right, ok) = insert_rec(node.right, data);
            node.right = new_right;
            inserted = ok;
        } else {
            return (Some(node), false);
        }
        node.update_height();
        let balance = node.balance_factor();
        if balance > 1 {
            if let Some(left) = &node.left {
                if data < &left.data {
                    return (Some(node).right_rotate(), true);
                }
                if data > &left.data {
                    node.left = node.left.left_rotate();
                    return (Some(node).right_rotate(), true);
                }
            }
        }
        if balance < -1 {
            if let Some(right) = &node.right {
                if data > &right.data {
                    return (Some(node).left_rotate(), true);
                }
                if data < &right.data {
                    node.right = node.right.right_rotate();
                    return (Some(node).left_rotate(), true);
                }
            }
        }
        (Some(node), inserted)
    } else {
        (Node::new(data.clone()).into(), true)
    }
}

fn delete_rec<T>(ptr: NodePtr<T>, data: &T) -> (NodePtr<T>, bool)
where 
    T: Clone + Ord + PartialOrd + Eq,
{
    if let Some(mut node) = ptr {
        let deleted: bool;
        if data < &node.data {
            let (new_left, ok) = delete_rec(node.left, data);
            node.left = new_left;
            deleted = ok;
        } else if data > &node.data {
            let (new_right, ok) = delete_rec(node.right, data);
            node.right = new_right;
            deleted = ok;
        } else {
            deleted = true;
            if node.left.is_none() || node.right.is_none() {
                return (node.left.or(node.right), true);
            } else {
                let tmp = node.right.min_value();
                node.data = tmp.unwrap().data.clone();
                let (new_right, _) = delete_rec(node.right, &node.data);
                node.right = new_right;
            }
        }
        node.update_height();
        let balance = node.balance_factor();
        if let Some(ref left) = node.left {
            if balance > 1 && left.balance_factor() >= 0 {
                return (Some(node).right_rotate(), deleted);
            }
            if balance > 1 && left.balance_factor() < 0 {
                node.left = node.left.left_rotate();
                return (Some(node).right_rotate(), deleted);
            }
        }
        if let Some(ref right) = node.right {
            if balance < -1 && right.balance_factor() <= 0 {
                return (Some(node).left_rotate(), deleted);
            }
            if balance < -1 && right.balance_factor() > 0 {
                node.right = node.right.right_rotate();
                return (Some(node).left_rotate(), deleted);
            }
        }
        (Some(node), deleted)
    } else {
        (None, false)
    }
}

trait Rotate<T>
where
    T: Clone + Ord + PartialOrd + Eq,
{
    fn left_rotate(self) -> Self;
    fn right_rotate(self) -> Self;
    fn min_value(&mut self) -> Option<&mut Node<T>>;
}

impl<T> Rotate<T> for NodePtr<T>
where
    T: Clone + Ord + PartialOrd + Eq,
{
    fn left_rotate(mut self) -> Self {
        let mut x = self.take().unwrap();
        let mut y = x.right.take().unwrap();
        x.right = y.left.take();
        y.left = Some(x);
        if let Some(left) = &mut y.left {
            left.update_height();
        }
        y.update_height();
        Some(y)
    }
    fn right_rotate(mut self) -> Self {
        let mut y = self.take().unwrap();
        let mut x = y.left.take().unwrap();
        y.left = x.right.take();
        x.right = Some(y);
        if let Some(right) = &mut x.right {
            right.update_height();
        }
        x.update_height();
        Some(x)
    }
    fn min_value(&mut self) -> Option<&mut Node<T>> {
        let mut ptr = self.as_mut()?;
        loop {
            if ptr.left.is_none() {
                return Some(ptr);
            }
            let next = ptr.left.as_mut().unwrap();
            ptr = next;
        }
    }
}

impl<T> Node<T>
where
    T: Clone + Ord + PartialOrd + Eq,
{
    pub fn new(data: T) -> Self {
        Self {
            data,
            left: None,
            right: None,
            height: 1,
        }
    }
    fn balance_factor(&self) -> i8 {
        height(&self.left) - height(&self.right)
    }
    fn update_height(&mut self) {
        self.height = 1 + height(&self.left).max(height(&self.right));
    }
}

pub struct AVLTree<T>
where
    T: Clone + Ord + PartialOrd + Eq,
{
    root: NodePtr<T>,
    size: usize,
}

impl<T> AVLTree<T>
where
    T: Clone + Ord + PartialOrd + Eq,
{
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }
    pub fn from<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut tree = Self::new();
        tree.extend(iter);
        tree
    }
}

impl<T> AVLTree<T>
where
    T: Clone + Ord + PartialOrd + Eq,
{
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn insert(&mut self, data: T) -> bool {
        let (new_root, inserted) = insert_rec(self.root.take(), &data);
        self.root = new_root;
        if inserted {
            self.size += 1;
        }
        inserted
    }
    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        iter.into_iter().for_each(|item| {
            self.insert(item);
        } );
    }
    pub fn contains(&self, data: &T) -> bool {
        let mut ptr = &self.root;
        while let Some(node) = ptr {
            if data < &node.data {
                ptr = &node.left;
            } else if data > &node.data {
                ptr = &node.right;
            } else {
                return true;
            }
        }
        false
    }
    pub fn remove(&mut self, data: &T) -> bool {
        let (new_root, deleted) = delete_rec(self.root.take(), data);
        self.root = new_root;
        if deleted {
            self.size -= 1;
        }
        deleted
    }
    pub fn get_min(&self) -> Option<&T> {
        let mut ptr = &self.root;
        while let Some(node) = ptr {
            if node.left.is_none() {
                return Some(&node.data);
            } else {
                ptr = &node.left;
            }
        }
        None
    }
    pub fn get_max(&self) -> Option<&T> {
        let mut ptr = &self.root;
        while let Some(node) = ptr {
            if node.right.is_none() {
                return Some(&node.data);
            } else {
                ptr = &node.right;
            }
        }
        None
    }
    pub fn get_floor(&self, floor: &T) -> Option<&T> {
        let mut ptr = &self.root;
        let mut candidate = None;
        while let Some(node) = ptr {
            if &node.data == floor {
                return Some(&node.data);
            } else if &node.data > floor {
                ptr = &node.left;
            } else {
                candidate = Some(&node.data);
                ptr = &node.right;
            }
        }
        candidate
    }
    pub fn get_ceil(&self, ceil: &T) -> Option<&T> {
        let mut ptr = &self.root;
        let mut candidate = None;
        while let Some(node) = ptr {
            if &node.data == ceil {
                return Some(&node.data);
            } else if &node.data < ceil {
                ptr = &node.right;
            } else {
                candidate = Some(&node.data);
                ptr = &node.left;
            }
        }
        candidate
    }
    pub fn clear(&mut self) {
        self.root = None;
        self.size = 0;
    }
}

impl<T> Debug for AVLTree<T>
where
    T: Debug + Clone + Ord + PartialOrd + Eq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut iter = Iter::new(&self.root);
        while let Some(item) = iter.next() {
            write!(f, "{:?}", item)?;
            if iter.has_next() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

impl<T> Clone for AVLTree<T>
where 
    T: Clone + Ord + PartialOrd + Eq
{
    fn clone(&self) -> Self {
        fn preorder_copy<T>(src: &NodePtr<T>) -> NodePtr<T>
        where
            T: Clone + Ord + PartialOrd + Eq
        {
            if let Some(node) = src {
                let mut cpy = Node::new(node.data.clone());
                cpy.height = node.height;
                cpy.left = preorder_copy(&node.left);
                cpy.right = preorder_copy(&node.right);
                cpy.into()
            } else {
                None
            }
        }
        Self { root: preorder_copy(&self.root), size: self.size }
    }
}

pub struct Iter<'a, T>
where
    T: Clone + Ord + PartialOrd + Eq,
{
    stack: LinkedList<&'a Node<T>>,
}

pub struct IntoIter<T>
where
    T: Clone + Ord + PartialOrd + Eq,
{
    stack: Vec<Box<Node<T>>>,
}

impl<'a, T> Iter<'a, T>
where
    T: Clone + Ord + PartialOrd + Eq,
{
    fn new(root: &'a NodePtr<T>) -> Self {
        let mut iter = Iter {
            stack: LinkedList::new(),
        };
        iter.push_left_branch(root);
        iter
    }
    fn push_left_branch(&mut self, mut ptr: &'a NodePtr<T>) {
        while let Some(node) = ptr.as_ref() {
            self.stack.push_head(node);
            ptr = &node.left;
        }
    }
    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

impl<T> IntoIter<T>
where
    T: Clone + Ord + PartialOrd + Eq,
{
    fn new(root: NodePtr<T>) -> Self {
        let mut iter = IntoIter { stack: Vec::new() };
        iter.push_left(root);
        iter
    }
    fn push_left(&mut self, mut node: Option<Box<Node<T>>>) {
        while let Some(mut n) = node {
            let right = n.right.take();
            self.stack.push(n);
            node = right;
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Clone + Ord + PartialOrd + Eq,
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            self.push_left_branch(&node.right);
            Some(&node.data)
        } else {
            None
        }
    }
}

impl<T> Iterator for IntoIter<T>
where
    T: Clone + Ord + PartialOrd + Eq,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let mut node = self.stack.pop()?;
        let val = node.data;
        if let Some(left) = node.left.take() {
            self.stack.push(left);
        }
        Some(val)
    }
}

impl<T> AVLTree<T>
where
    T: Clone + Ord + PartialOrd + Eq,
{
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter::new(&self.root)
    }
}

impl<T> IntoIterator for AVLTree<T>
where
    T: Clone + Ord + PartialOrd + Eq,
{
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.root)
    }
}

impl<T: Clone> FromIterator<T> for AVLTree<T>
where
    T: Clone + Ord + PartialOrd + Eq
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from(iter)
    }
}