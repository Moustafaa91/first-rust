
pub trait Collection<T> {
    fn count(&self) -> usize;
    fn clear(&mut self);
    fn memory_consumption(&self) -> usize;
}
