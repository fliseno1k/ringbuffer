pub trait Slice {
    type Element;

    fn slice(&self) -> &[Self::Element];
}

pub trait SliceMut: Slice {
    fn slice_mut(&mut self) -> &mut [Self::Element];
}

pub trait FixedSizeArray {
    const LEN: usize;
}

impl<'a, T> Slice for &'a [T] {
    type Element = T;

    #[inline]
    fn slice(&self) -> &[Self::Element] {
        self
    }
}

impl<'a, T> Slice for &'a mut [T] {
    type Element = T;

    #[inline]
    fn slice(&self) -> &[Self::Element] {
        self
    }
}

impl<'a, T> SliceMut for &'a mut [T] {
    #[inline]
    fn slice_mut(&mut self) -> &mut [Self::Element] {
        self
    }
}

impl<T> Slice for Box<[T]> {
    type Element = T;

    #[inline]
    fn slice(&self) -> &[Self::Element] {
        &self[..]
    }
}

impl<T> SliceMut for Box<[T]> {
    #[inline]
    fn slice_mut(&mut self) -> &mut [Self::Element] {
        &mut self[..]
    }
}

impl<T> Slice for Vec<T> {
    type Element = T;

    #[inline]
    fn slice(&self) -> &[Self::Element] {
        &self[..]
    }
}

impl<T> SliceMut for Vec<T> {
    #[inline]
    fn slice_mut(&mut self) -> &mut [Self::Element] {
        &mut self[..]
    }
}

impl<T, const N: usize> Slice for [T; N] {
    type Element = T;

    #[inline]
    fn slice(&self) -> &[Self::Element] {
        &self[..]
    }
}

impl<T, const N: usize> SliceMut for [T; N] {
    #[inline]
    fn slice_mut(&mut self) -> &mut [Self::Element] {
        &mut self[..]
    }
}

impl<T, const N: usize> FixedSizeArray for [T; N] {
    const LEN: usize = N;
}
