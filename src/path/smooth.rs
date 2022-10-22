use crate::geometry::Mix;
use crate::path::{WithNeighbours, WithNeighboursIterator};
use num_traits::one;

pub struct SmoothIterator<I>
where
    I: Iterator,
    I::Item: Mix + Clone,
{
    iterator: WithNeighboursIterator<I>,
}

impl<I> SmoothIterator<I>
where
    I: Iterator,
    I::Item: Mix + Clone,
{
    pub fn new(iterator: I) -> Self {
        Self {
            iterator: WithNeighboursIterator::new(iterator),
        }
    }
}

impl<I> Iterator for SmoothIterator<I>
where
    I: Iterator,
    I::Item: Mix + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.iterator.next()? {
            (Some(p1), p2, Some(p3)) => {
                let one = one::<<<I as Iterator>::Item as Mix>::Fraction>();
                let half: <<I as Iterator>::Item as Mix>::Fraction = one / (one + one);
                p1.mix(p3, half).mix(p2, half)
            }
            (_, p2, _) => p2,
        })
    }
}
