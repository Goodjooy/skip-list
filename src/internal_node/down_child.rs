use std::ptr::NonNull;

use crate::leaf_node::LeafNode;

use super::InternalNode;

pub enum DownType<T> {
    Internal(NonNull<InternalNode<T>>),
    Leaf(NonNull<LeafNode<T>>),
}

impl<T> DownType<T> {
    pub(crate) fn as_brother(&mut self) {
        match self {
            DownType::Internal(ptr) => {
                if let Some(next) = &mut unsafe { ptr.as_mut() }.next_ptr {
                    next.as_brother()
                }
            }
            DownType::Leaf(ptr) => {
                if let Some(next) = &mut unsafe { ptr.as_mut() }.next_ptr {
                    next.as_brother()
                }
            }
        };
    }
}

impl<T> InternalNode<T> {
    pub(crate) fn child(&self) -> &DownType<T> {
        &self.payload.0
    }

    pub(crate) fn child_mut(&mut self) -> &mut DownType<T> {
        &mut self.payload.0
    }
}
