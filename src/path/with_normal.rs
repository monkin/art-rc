use crate::geometry::{Distance, Mix, Normal, Vector};
use crate::path::WithNeighboursIterator;
use num_traits::Zero;

#[derive(Debug)]
pub struct WithNormal<T> {
    pub point: T,
    pub normal: Vector,
}

impl<T: Clone> Clone for WithNormal<T> {
    fn clone(&self) -> Self {
        Self {
            point: self.point.clone(),
            normal: self.normal,
        }
    }
}

impl<T: Copy> Copy for WithNormal<T> {}

impl<T> WithNormal<T> {
    pub fn normalize(self) -> Self {
        Self {
            point: self.point,
            normal: self.normal.normalize(),
        }
    }
}

pub struct WithNormalIterator<I>
where
    I: Iterator,
    I::Item: Copy + Normal,
{
    iterator: WithNeighboursIterator<I>,
}

impl<I> WithNormalIterator<I>
where
    I: Iterator,
    I::Item: Copy + Normal,
{
    pub fn new(iterator: I) -> Self {
        Self {
            iterator: WithNeighboursIterator::new(iterator),
        }
    }
}

impl<I> Iterator for WithNormalIterator<I>
where
    I: Iterator,
    I::Item: Copy + Normal,
{
    type Item = WithNormal<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let point = self.iterator.next()?;
        Some(WithNormal {
            point: point.1,
            normal: match point {
                (Some(p1), _, Some(p3)) => Normal::normal(p1, p3).normalize(),
                (_, p2, Some(p3)) => Normal::normal(p2, p3).normalize(),
                (Some(p1), p2, _) => Normal::normal(p1, p2).normalize(),
                _ => Vector::zero(),
            },
        })
    }
}

impl<T: Distance> Distance for WithNormal<T> {
    type Output = T::Output;

    fn distance(self, other: Self) -> Self::Output {
        self.point.distance(other.point)
    }
}

impl<T> Mix for WithNormal<T>
where
    T: Mix<Fraction = f32>,
{
    type Fraction = T::Fraction;

    fn mix(self, other: Self, t: Self::Fraction) -> Self {
        Self {
            point: self.point.mix(other.point, t),
            normal: self.normal.mix(other.normal, t),
        }
    }
}
