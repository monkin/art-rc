pub type WithNeighbours<T> = (Option<T>, T, Option<T>);

#[derive(Debug)]
pub struct WithNeighboursIterator<I>
where
    I: Iterator,
    I::Item: Clone,
{
    iterator: I,
    state: (Option<I::Item>, Option<I::Item>),
}

impl<I> WithNeighboursIterator<I>
where
    I: Iterator,
    I::Item: Clone,
{
    pub fn new(mut iterator: I) -> Self {
        Self {
            state: (None, iterator.next()),
            iterator,
        }
    }
}

impl<I> Iterator for WithNeighboursIterator<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = WithNeighbours<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.state.1.as_ref()?.clone();
        let previous = self.state.0.clone();
        let next = self.iterator.next();
        self.state = (Some(current.clone()), next.clone());
        Some((previous, current, next))
    }
}

#[cfg(test)]
mod test {
    use crate::path::WithNeighboursIterator;

    #[test]
    fn simple() {
        let data: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
        let result: Vec<_> = WithNeighboursIterator::new(data.into_iter()).collect();
        assert_eq!(
            result,
            vec![
                (None, 1.0, Some(2.0)),
                (Some(1.0), 2.0, Some(3.0)),
                (Some(2.0), 3.0, Some(4.0)),
                (Some(3.0), 4.0, None)
            ]
        )
    }

    #[test]
    fn single() {
        let data: Vec<f32> = vec![1.0];
        let result: Vec<_> = WithNeighboursIterator::new(data.into_iter()).collect();
        assert_eq!(result, vec![(None, 1.0, None)])
    }
}
