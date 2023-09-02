use std::{marker::PhantomData, ptr::NonNull, sync::Arc};

use crate::{node::SkipNode, SkipList};

impl<T> SkipList<T> {
    pub fn iter(&self) -> SkipListRefIter<'_, T> {
        let ptr = if let Some(head) = self.head {
            Some(SkipNode::go_bottom_raw(head))
        } else {
            None
        };

        SkipListRefIter {
            ptr,
            _phantom: PhantomData,
        }
    }
}

pub struct SkipListRefIter<'a, T> {
    ptr: Option<NonNull<SkipNode<T>>>,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> Iterator for SkipListRefIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.ptr {
            Some(ptr) => {
                let ptr = unsafe { ptr.as_ref() };
                self.ptr = ptr.next;
                let item = ptr.get_raw_item();
                Some(item)
            }
            None => None,
        }
    }
}

impl<T> IntoIterator for SkipList<T> {
    type Item = Arc<T>;

    type IntoIter = SkipListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        SkipListIter { inner: self }
    }
}

pub struct SkipListIter<T> {
    inner: SkipList<T>,
}

impl<T> Iterator for SkipListIter<T> {
    type Item = Arc<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.pop_head()
    }
}
