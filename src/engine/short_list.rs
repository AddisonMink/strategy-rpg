const MAX_SHORT_LIST_SIZE: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShortList<T> {
    pub items: [T; MAX_SHORT_LIST_SIZE],
    pub len: usize,
}

impl<T: Copy> ShortList<T> {
    pub const fn new(items: &[T]) -> Self {
        let mut arr = [items[0]; MAX_SHORT_LIST_SIZE];
        let mut i = 0;
        while i < items.len() && i < MAX_SHORT_LIST_SIZE {
            arr[i] = items[i];
            i += 1;
        }
        Self { items: arr, len: i }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.items[..self.len]
    }
}
