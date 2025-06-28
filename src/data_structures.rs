use std::mem;

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

    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.items.shrink_to_fit();
    }

    pub fn memory_consumption(&self) -> usize {
        self.items.capacity() * mem::size_of::<T>() + mem::size_of_val(&self.items)
    }
}

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

    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.items.shrink_to_fit();
    }

    pub fn memory_consumption(&self) -> usize {
        self.items.capacity() * mem::size_of::<T>() + mem::size_of_val(&self.items)
    }
}