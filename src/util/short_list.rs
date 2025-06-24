const MAX_SHORT_LIST_SIZE: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShortList<T> {
    items: Option<[T; MAX_SHORT_LIST_SIZE]>,
    len: usize,
}

impl<T: Copy> ShortList<T> {
    pub const fn empty() -> Self {
        Self {
            items: None,
            len: 0,
        }
    }

    pub const fn new(items: &[T]) -> Self {
        let mut arr = [items[0]; MAX_SHORT_LIST_SIZE];
        let mut i = 0;
        while i < items.len() && i < MAX_SHORT_LIST_SIZE {
            arr[i] = items[i];
            i += 1;
        }
        Self {
            items: Some(arr),
            len: i,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (if let Some(items) = &self.items {
            &items[..self.len]
        } else {
            &[]
        })
        .iter()
    }
}
