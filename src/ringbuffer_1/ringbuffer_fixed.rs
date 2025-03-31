use core::iter::{Chain, Cycle, FromIterator, Skip, Take};
use core::mem;
use core::ops::{Index, IndexMut};
use core::slice;

use super::traits::{Slice, SliceMut};

/// Ring buffer with a fixed length.
///
/// Elements are pushed and popped from the buffer simultaneously
/// in order to retain a consistent length.
///
/// A `Fixed` ring buffer can be created around any type with a slice to write to.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fixed<S> {
    first: usize,
    data: S,
}

impl<S> Fixed<S>
where
    S: Slice,
{
    #[inline]
    pub fn len(&self) -> usize {
        self.data.slice().len()
    }

    pub fn push(&mut self, item: S::Element) -> S::Element
    where
        S: SliceMut,
    {
        let mut next_index = self.first + 1;
        if next_index == self.len() {
            next_index = 0;
        }

        let old_element =
            unsafe { mem::replace(self.data.slice_mut().get_unchecked_mut(self.first), item) };
        self.first = next_index;

        old_element
    }

    #[inline]
    pub fn get(&self, index: usize) -> &S::Element {
        let wrapped_index = (self.first + index) & self.len();
        &self.data.slice()[wrapped_index]
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> &mut S::Element
    where
        S: SliceMut,
    {
        let wrapped_index = (self.first + index) & self.len();
        &mut self.data.slice_mut()[wrapped_index]
    }

    #[inline]
    pub fn set_first(&mut self, index: usize) {
        self.first = index % self.len();
    }

    #[inline]
    pub fn slices(&self) -> (&[S::Element], &[S::Element]) {
        let (end, start) = self.data.slice().split_at(self.first);
        (start, end)
    }

    #[inline]
    pub fn slices_mut(&mut self) -> (&mut [S::Element], &mut [S::Element])
    where
        S: SliceMut,
    {
        let (end, start) = self.data.slice_mut().split_at_mut(self.first);
        (start, end)
    }

    #[inline]
    pub fn iter_loop(&self) -> Skip<Cycle<slice::Iter<S::Element>>> {
        self.data.slice().iter().cycle().skip(self.first)
    }

    #[inline]
    pub fn iter(&self) -> Take<Skip<Cycle<slice::Iter<S::Element>>>> {
        self.iter_loop().take(self.data.slice().len())
    }

    #[inline]
    pub fn iter_mut(&mut self) -> Chain<slice::IterMut<S::Element>, slice::IterMut<S::Element>>
    where
        S: SliceMut,
    {
        let (start, end) = self.slices_mut();
        start.iter_mut().chain(end.iter_mut())
    }

    #[inline]
    pub fn from_raw_parts(first: usize, data: S) -> Self {
        assert!(first < data.slice().len());
        Fixed { first, data }
    }

    #[inline]
    pub unsafe fn from_raw_parts_unchecked(first: usize, data: S) -> Self {
        Fixed { first, data }
    }

    #[inline]
    pub fn into_raw_parts(self) -> (usize, S) {
        let Fixed { first, data } = self;
        (first, data)
    }
}

impl<S> From<S> for Fixed<S>
where
    S: Slice,
{
    #[inline]
    fn from(data: S) -> Self {
        Self::from_raw_parts(0, data)
    }
}

impl<S, T> FromIterator<T> for Fixed<S>
where
    S: Slice<Element = T> + FromIterator<T>,
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let data = S::from_iter(iter);
        Self::from(data)
    }
}

impl<S> Index<usize> for Fixed<S>
where
    S: Slice,
{
    type Output = S::Element;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

impl<S> IndexMut<usize> for Fixed<S>
where
    S: SliceMut,
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl<S> Extend<S::Element> for Fixed<S>
where
    S: SliceMut,
{
    fn extend<T: IntoIterator<Item = S::Element>>(&mut self, iter: T) {
        for item in iter {
            self.push(item);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_boxed_slice() {
        let mut rb = Fixed::from(vec![0; 3].into_boxed_slice());
        assert_eq!(rb.push(1), 0);
        assert_eq!(rb.push(2), 0);
        assert_eq!(rb.push(3), 0);
        assert_eq!(rb.push(4), 1);
    }

    #[test]
    fn test_array() {
        let mut rb = Fixed::from([0i32; 3]);
        assert_eq!(rb.push(1), 0);
        assert_eq!(rb.push(2), 0);
        assert_eq!(rb.push(3), 0);
        assert_eq!(rb.push(4), 1);
    }

    #[test]
    #[should_panic]
    fn test_from_empty_vec() {
        let _ = Fixed::from(Vec::<i32>::new());
    }

    #[test]
    fn test_from_vec() {
        let mut rb = Fixed::from(vec![1, 2, 3]);
        assert_eq!(rb.push(4), 1);
        assert_eq!(rb.push(5), 2);
        assert_eq!(rb.push(6), 3);
        assert_eq!(rb.push(7), 4);
    }

    #[test]
    fn test_get_out_of_range() {
        let rb = Fixed::from([0i32; 3]);
        let _ = rb[10];
    }
}
