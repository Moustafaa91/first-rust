use std::mem;
use crate::data_structures::Collection;

pub struct Queue<T> {
    items: Vec<T>,
    count: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            items: Vec::new(),
            count: 0,
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push(item);
        self.count += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if !self.items.is_empty() {
            self.count -= 1;
            Some(self.items.remove(0))
        } else {
            None
        }
    }
}

impl<T> Collection<T> for Queue<T> {
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