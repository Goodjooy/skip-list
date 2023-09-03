use std::{ops::Deref, sync::Arc};

use super::InternalNode;

#[derive(Debug, PartialEq, Eq)]
pub struct Range<T> {
    pub(crate) min: Arc<T>,
    pub(crate) max: Arc<T>,
}

impl<T> Clone for Range<T> {
    fn clone(&self) -> Self {
        Self {
            min: self.min.clone(),
            max: self.max.clone(),
        }
    }
}

impl<T: Ord> Range<T> {
    pub(crate) fn new(value: Arc<T>) -> Self {
        Self {
            min: value.clone(),
            max: value,
        }
    }
    #[cfg(test)]
    pub(crate) fn new_in(min: T, max: T) -> Self {
        Self {
            min: Arc::new(min),
            max: Arc::new(max),
        }
    }

    pub(crate) fn in_range(&self, value: &T) -> bool {
        value >= self.min.deref() && value <= self.max.deref()
    }

    pub(crate) fn left_range(&self, value: &T) -> bool {
        value < self.min.deref()
    }

    pub(crate) fn update(&mut self, value: Arc<T>) {
        if value > self.max {
            self.max = value;
        } else if value < self.min {
            self.min = value;
        }
    }
    pub(crate) fn combine(&mut self, rhs: &Self) {
        if rhs.min < self.min {
            self.min = rhs.min.clone()
        }
        if rhs.max > self.max {
            self.max = rhs.max.clone();
        }
    }
}
#[cfg(test)]
mod test {
    use std::sync::Arc;

    use super::Range;
    #[test]
    fn test_update_upper() {
        let mut range = Range::new_in(4, 4);
        range.update(Arc::new(15i32));

        assert_eq!(&*range.min, &4);
        assert_eq!(&*range.max, &15);
    }

    #[test]
    fn test_update_lower() {
        let mut range = Range::new_in(4, 4);
        range.update(Arc::new(1i32));

        assert_eq!(&*range.min, &1);
        assert_eq!(&*range.max, &4);
    }
}

impl<T> InternalNode<T> {
    pub fn range(&self) -> &Range<T> {
        &self.payload.1
    }
    pub fn range_mut(&mut self) -> &mut Range<T> {
        &mut self.payload.1
    }
}
