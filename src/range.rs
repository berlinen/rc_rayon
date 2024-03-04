use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Iter<T> {
    range: Range<T>,
}

impl<T> IntoParallelIterator for Range<T> where Iter<T>: ParallelIterator {}
