#[derive(Debug)]
pub struct DeduplicateIterator<I>
where
    I: Iterator,
    I::Item: Clone + PartialEq,
{
    iterator: I,
    previous: Option<I::Item>,
}

impl<I> DeduplicateIterator<I>
where
    I: Iterator,
    I::Item: Clone + PartialEq,
{
    pub fn new(iterator: I) -> Self {
        Self {
            iterator,
            previous: None,
        }
    }
}

impl<I> Iterator for DeduplicateIterator<I>
where
    I: Iterator,
    I::Item: Clone + PartialEq,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = self.iterator.next();
        while result.is_some() && result == self.previous {
            result = self.iterator.next();
        }
        self.previous = result.clone();
        result
    }
}
