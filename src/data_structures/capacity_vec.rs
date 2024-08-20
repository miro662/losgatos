use core::{
    mem,
    ops::{Deref, DerefMut, Index, IndexMut},
};

// Stack-allocated vector with given capacity
#[derive(Clone)]
pub struct CapacityVec<T: Sized, const CAPACITY: usize> {
    len: usize,
    data: [T; CAPACITY],
}

impl<T: Sized, const CAPACITY: usize> CapacityVec<T, CAPACITY> {
    pub fn empty() -> Self {
        CapacityVec {
            len: 0,
            // SAFETY: if len == 0, this memory is never accessed
            data: unsafe { mem::zeroed() },
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, item: T) {
        if self.len + 1 >= CAPACITY {
            panic!("Cannot push item: capacity {} exceeded", CAPACITY);
        }

        self.data[self.len] = item;
        self.len += 1;
    }
}

impl<T: Sized, const CAPACITY: usize> FromIterator<T> for CapacityVec<T, CAPACITY> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut result = Self::empty();

        for it in iter.into_iter().take(CAPACITY) {
            result.data[result.len] = it;
            result.len += 1;
        }

        result
    }
}

impl<T: Sized, const CAPACITY: usize> IntoIterator for CapacityVec<T, CAPACITY> {
    type Item = T;

    type IntoIter = core::iter::Take<core::array::IntoIter<T, CAPACITY>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter().take(self.len)
    }
}

impl<T: Sized, const CAPACITY: usize> Deref for CapacityVec<T, CAPACITY> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data[0..self.len]
    }
}

impl<T: Sized, const CAPACITY: usize> DerefMut for CapacityVec<T, CAPACITY> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data[0..self.len]
    }
}

impl<T: Sized, const CAPACITY: usize> Index<usize> for CapacityVec<T, CAPACITY> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("{} out of bounds (len = {})", index, self.len)
        }
        &self.data[index]
    }
}

impl<T: Sized, const CAPACITY: usize> IndexMut<usize> for CapacityVec<T, CAPACITY> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("{} out of bounds (len = {})", index, self.len)
        }
        &mut self.data[index]
    }
}
