use crate::geometry::{Distance, Mix};
use crate::path::{SegmentsIterator, WithNeighbours, WithNeighboursIterator, WithOffsetIterator};
use num_traits::zero;
use std::fmt::Debug;

#[derive(Debug)]
pub struct SplitIterator<I>
where
    I: Iterator,
    I::Item: Mix + Distance + Clone + Debug,
    <I::Item as Distance>::Output: Debug,
{
    iterator: WithNeighboursIterator<WithOffsetIterator<I>>,
    segment: Option<<WithNeighboursIterator<WithOffsetIterator<I>> as Iterator>::Item>,
    offset: <I::Item as Distance>::Output,
    step: <I::Item as Distance>::Output,
}

impl<I> SplitIterator<I>
where
    I: Iterator,
    I::Item: Mix + Distance + Clone + Debug,
    <I::Item as Distance>::Output: Debug + Into<<I::Item as Mix>::Fraction>,
{
    pub fn new(iterator: I, step: <I::Item as Distance>::Output) -> Self {
        Self {
            iterator: WithNeighboursIterator::new(WithOffsetIterator::new(iterator)),
            segment: None,
            step,
            offset: zero(),
        }
    }
}

impl<I> Iterator for SplitIterator<I>
where
    I: Iterator,
    I::Item: Mix + Distance + Clone + Debug,
    <I::Item as Distance>::Output: Debug + Into<<I::Item as Mix>::Fraction>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.offset;
        let mut segment = self.segment.clone().or_else(|| self.iterator.next())?;

        while segment.1.offset <= offset && segment.2.is_some() {
            segment = self.iterator.next()?;
        }
        println!("test: {:?}", segment);

        self.segment = Some(segment.clone());
        self.offset = self.offset + self.step;

        return match segment {
            (None, point, None) => {
                self.segment = None;
                Some(point.point)
            }
            (Some(p1), p2, _) => {
                if p2.offset <= offset {
                    self.segment = None;
                    Some(p2.point)
                } else {
                    let r = p1.point.mix(
                        p2.point,
                        ((offset - p1.offset) / (p2.offset - p1.offset)).into(),
                    );
                    Some(r)
                }
            }
            _ => None,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::SplitIterator;
    use crate::geometry::Vector;

    #[test]
    fn single() {
        let split: Vec<_> =
            SplitIterator::new(vec![Vector::new(1.0, 1.0)].into_iter(), 0.5).collect();

        assert_eq!(split, vec![Vector::new(1.0, 1.0)])
    }

    #[test]
    fn horizontal_line() {
        let points: Vec<_> = SplitIterator::new(
            vec![Vector::new(0.0, 0.0), Vector::new(2.0, 0.0)].into_iter(),
            0.5,
        )
        .collect();

        assert_eq!(
            points,
            vec![
                Vector::new(0.0, 0.0),
                Vector::new(0.5, 0.0),
                Vector::new(1.0, 0.0),
                Vector::new(1.5, 0.0),
                Vector::new(2.0, 0.0)
            ]
        );
    }

    #[test]
    fn not_ceil_length() {
        let points: Vec<_> = SplitIterator::new(
            vec![Vector::new(0.0, 0.0), Vector::new(1.25, 0.0)].into_iter(),
            0.5,
        )
        .collect();

        assert_eq!(
            points,
            vec![
                Vector::new(0.0, 0.0),
                Vector::new(0.5, 0.0),
                Vector::new(1.0, 0.0),
                Vector::new(1.25, 0.0),
            ]
        );
    }
}
