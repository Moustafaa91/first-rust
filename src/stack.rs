use std::mem;
use crate::data_structures::Collection;

pub struct Stack<T> {
    items: Vec<T>,
    count: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            items: Vec::new(),
            count: 0, // may need it later for tracking the number of items
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
        self.count += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if !self.items.is_empty() {
            self.count -= 1;
            self.items.pop()
        } else {
            None
        }
    }
}

impl<T> Collection<T> for Stack<T> {
    fn count(&self) -> usize {
        self.items.len()
    }

    fn clear(&mut self) {
        self.items.clear();
        self.items.shrink_to_fit();
    }

    fn memory_consumption(&self) -> usize {
        self.items.capacity() * mem::size_of::<T>() + mem::size_of_val(&self.items)
    }
}