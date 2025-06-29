use std::collections::VecDeque;

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

    pub fn from_vecdeque(vec: VecDeque<T>) -> Self {
        let mut arr = [vec[0]; MAX_SHORT_LIST_SIZE];
        let len = vec.len().min(MAX_SHORT_LIST_SIZE);
        for i in 0..len {
            arr[i] = vec[i];
        }
        Self {
            items: Some(arr),
            len,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            self.items.as_ref().and_then(|items| items.get(index))
        } else {
            None
        }
    }

    pub fn last(&self) -> Option<&T> {
        if self.len > 0 {
            self.items
                .as_ref()
                .and_then(|items| items.get(self.len - 1))
        } else {
            None
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
