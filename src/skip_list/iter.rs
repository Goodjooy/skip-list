use std::{marker::PhantomData, ptr::NonNull};

use crate::leaf_node::LeafNode;

use super::SkipLinkedList;

pub struct SkipLinkedListRefIter<'r, T> {
    ptr: Option<NonNull<LeafNode<T>>>,
    __phantom: PhantomData<&'r T>,
}

impl<'r, T> Iterator for SkipLinkedListRefIter<'r, T> {
    type Item = &'r T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.ptr {
            Some(ptr) => {
                let ptr = unsafe { ptr.as_ref() };
                self.ptr = ptr.next_ptr.map(|next| next.next());
                Some(ptr.value())
            }
            None => None,
        }
    }
}

impl<T> SkipLinkedList<T> {
    pub fn iter(&self) -> SkipLinkedListRefIter<'_, T> {
        SkipLinkedListRefIter {
            ptr: self.head.map(|ptr| unsafe { ptr.as_ref() }.go_leaf_raw()),
            __phantom: PhantomData,
        }
    }
}
