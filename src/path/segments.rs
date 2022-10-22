#[derive(Debug)]
pub struct SegmentsIterator<I>
where
    I: Iterator,
    I::Item: Clone,
{
    iterator: I,
    previous: Option<I::Item>,
}

impl<I> SegmentsIterator<I>
where
    I: Iterator,
    I::Item: Clone,
{
    pub fn new(iterator: I) -> Self {
        Self {
            iterator,
            previous: None,
        }
    }
}

impl<I> Iterator for SegmentsIterator<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let previous = self
            .previous
            .as_ref()
            .map(|v| v.clone())
            .or_else(|| self.iterator.next())?;
        let next = self.iterator.next()?;
        self.previous = Some(next.clone());
        Some((previous, next))
    }
}

#[cfg(test)]
mod tests {
    use super::SegmentsIterator;

    #[test]
    fn basic() {
        let segments: Vec<(f32, f32)> =
            SegmentsIterator::new(vec![0.0, 1.0, 2.0, 3.0].into_iter()).collect();
        assert_eq!(segments, vec![(0.0, 1.0), (1.0, 2.0), (2.0, 3.0)]);
    }
}
