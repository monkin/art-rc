use crate::geometry::Distance;
use num_traits::real::Real;
use num_traits::zero;
use std::fmt::Debug;

#[derive(Debug)]
pub struct WithOffset<T: Distance> {
    pub point: T,
    pub offset: T::Output,
}

impl<T: Distance + Clone> Clone for WithOffset<T> {
    fn clone(&self) -> Self {
        Self {
            point: self.point.clone(),
            offset: self.offset,
        }
    }
}

impl<T: Distance + Copy> Copy for WithOffset<T> {}

impl<T: Distance> Distance for WithOffset<T> {
    type Output = T::Output;

    fn distance(self, other: Self) -> Self::Output {
        (self.offset - other.offset).abs()
    }
}

#[derive(Debug)]
pub struct WithOffsetIterator<I>
where
    I: Iterator,
    I::Item: Distance + Clone,
    <I::Item as Distance>::Output: Debug,
{
    iterator: I,
    previous: Option<WithOffset<I::Item>>,
}

impl<I> WithOffsetIterator<I>
where
    I: Iterator,
    I::Item: Distance + Clone,
    <I::Item as Distance>::Output: Debug,
{
    pub fn new(iterator: I) -> Self {
        Self {
            iterator,
            previous: None,
        }
    }
}

impl<I> Iterator for WithOffsetIterator<I>
where
    I: Iterator,
    I::Item: Distance + Clone,
    <I::Item as Distance>::Output: Debug,
{
    type Item = WithOffset<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let point = self.iterator.next()?;
        let result = Some(match &self.previous {
            None => WithOffset {
                point,
                offset: zero(),
            },
            Some(previous) => WithOffset {
                offset: previous.offset + previous.point.clone().distance(point.clone()),
                point,
            },
        });
        self.previous = result.clone();
        result
    }
}
