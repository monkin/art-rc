use std::cell::RefCell;
use std::mem;
use std::ops::Deref;
use std::rc::Rc;

pub(crate) trait PoolRequest {
    type Item;
    type Error;

    fn distance(&self, item: &Self::Item) -> Option<f32>;
    fn prepare(&self, item: &mut Self::Item);
    fn create(&self) -> Result<Self::Item, Self::Error>;
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Pool<T> {
    items: Rc<RefCell<Vec<T>>>,
    max_size: usize,
}

impl<T> Pool<T> {
    pub fn new(max_size: usize) -> Self {
        Pool {
            items: Default::default(),
            max_size,
        }
    }

    pub fn size(&self) -> usize {
        self.items.borrow().len()
    }

    pub fn max_size(&self) -> usize {
        self.max_size
    }

    pub fn take<E>(
        &self,
        request: &impl PoolRequest<Item = T, Error = E>,
    ) -> Result<PoolEntry<T>, E> {
        let mut index = None;
        for i in 0..self.items.borrow().len() {
            let data = self.items.borrow();
            let d = request.distance(&data[i]);
            index = index
                .map(|index: (usize, f32)| d.map(|d| if d < index.1 { (i, d) } else { index }))
                .flatten()
                .or(d.map(|d| (i, d)));

            if d == Some(0.0) {
                break;
            }
        }

        let value = index
            .map(|i| {
                Ok({
                    let mut item = self.items.borrow_mut().remove(i.0);
                    request.prepare(&mut item);
                    item
                })
            })
            .unwrap_or_else(|| request.create())?;

        Ok(PoolEntry {
            pool: Pool {
                items: self.items.clone(),
                max_size: self.max_size,
            },
            value: Some(value),
        })
    }
    fn release(&self, value: T) {
        let mut data = self.items.borrow_mut();
        if data.len() >= self.max_size {
            data.remove(0);
        }
        data.push(value);
    }
}

#[derive(Debug)]
pub(crate) struct PoolEntry<T> {
    pool: Pool<T>,
    value: Option<T>,
}

impl<T: PartialEq> PartialEq<PoolEntry<T>> for PoolEntry<T> {
    fn eq(&self, other: &PoolEntry<T>) -> bool {
        self.value == other.value
    }
}

impl<T> Drop for PoolEntry<T> {
    fn drop(&mut self) {
        let value = mem::replace(&mut self.value, None);
        self.pool.release(value.unwrap());
    }
}

impl<T> Deref for PoolEntry<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value.as_ref().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::pool::{Pool, PoolRequest};

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    struct Item {
        value: i32,
        extra: i32,
    }

    struct ItemRequest {
        value: i32,
        extra: i32,
        fail: bool,
    }

    impl PoolRequest for ItemRequest {
        type Item = Item;
        type Error = i32;

        fn distance(&self, item: &Item) -> Option<f32> {
            Some((self.value - item.value).abs() as f32)
        }

        fn prepare(&self, item: &mut Item) {
            item.extra = self.extra;
        }

        fn create(&self) -> Result<Item, i32> {
            if !self.fail {
                Ok(Item {
                    value: self.value,
                    extra: self.extra,
                })
            } else {
                Err(-1)
            }
        }
    }

    #[test]
    fn returns_created_object() {
        let pool = Pool::new(4);
        assert_eq!(
            pool.take(&ItemRequest {
                value: 1,
                extra: 2,
                fail: false,
            })
            .map(|r| *r),
            Ok(Item { value: 1, extra: 2 })
        )
    }

    #[test]
    fn returns_cached_object() {
        let pool = Pool::new(4);
        let request = ItemRequest {
            value: 1,
            extra: 2,
            fail: false,
        };
        let v1 = pool.take(&request).map(|v| *v);
        let v2 = pool.take(&request).map(|v| *v);
        assert_eq!(v1, v2);
        assert_eq!(pool.size(), 1);
    }

    #[test]
    fn updates_cached_object() {
        let pool = Pool::new(4);
        let request1 = ItemRequest {
            value: 1,
            extra: 2,
            fail: false,
        };
        let v1 = pool.take(&request1).map(|v| *v);

        let request2 = ItemRequest {
            value: 1,
            extra: 3,
            fail: false,
        };
        let v2 = pool.take(&request2).map(|v| *v);

        assert_eq!(v1, Ok(Item { value: 1, extra: 2 }));
        assert_eq!(v2, Ok(Item { value: 1, extra: 3 }));
        assert_eq!(pool.size(), 1);
    }

    #[test]
    fn creates_second_object_if_the_fist_was_taken() {
        let pool = Pool::new(4);
        let request = ItemRequest {
            value: 1,
            extra: 2,
            fail: false,
        };
        {
            let v1 = pool.take(&request);
            let v2 = pool.take(&request);
            assert_eq!(v1, v2);
        };
        assert_eq!(pool.size(), 2);
    }

    #[test]
    fn stores_no_more_than_max_size_objects() {
        let pool = Pool::new(10);
        let request = ItemRequest {
            value: 1,
            extra: 2,
            fail: false,
        };
        {
            let items: Vec<_> = (0..33).into_iter().map(|_i| pool.take(&request)).collect();
            assert_eq!(items.len(), 33);
        };
        assert!(pool.size() >= 5);
        assert!(pool.size() <= 10);
    }

    #[test]
    fn return_error_on_creation_fail() {
        let pool = Pool::new(10);
        let request = ItemRequest {
            value: 1,
            extra: 2,
            fail: true,
        };

        assert_eq!(pool.take(&request), Err(-1));
    }
}
