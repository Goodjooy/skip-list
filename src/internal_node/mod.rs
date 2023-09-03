mod down_child;
mod insert;
mod range;
use std::ptr::NonNull;

use crate::{leaf_node::LeafNode, skip_node::SkipNode};
pub(crate) use down_child::DownType;
pub(crate) use range::Range;

pub(crate) type InternalNode<T> = SkipNode<(DownType<T>, Range<T>)>;

impl<T> InternalNode<T> {
    pub(crate) fn gen_range(&self) -> Range<T>
    where
        T: Ord,
    {
        let mut init = self.payload.1.clone();
        if let Some(next) = self.next_ptr {
            if let Some(ptr) = next.next_brother() {
                let max = unsafe { ptr.as_ref() }.gen_range();
                init.combine(&max);
            }
        }

        init
    }
    #[cfg(test)]
    pub(crate) fn height(&self) -> usize {
        match self.child() {
            DownType::Internal(ptr) => {
                let v = unsafe { ptr.as_ref().height() };
                v + 1
            }
            DownType::Leaf(_) => 1,
        }
    }

    pub(crate) fn children_num(&self) -> usize {
        match self.child() {
            DownType::Internal(ptr) => unsafe { ptr.as_ref() }.brother_num(),
            DownType::Leaf(leaf) => unsafe { leaf.as_ref().brother_num() },
        }
    }
    #[cfg(test)]
    pub(crate) fn go_leaf(&self) -> &LeafNode<T> {
        unsafe { self.go_leaf_raw().as_ref() }
    }

    pub(crate) fn go_leaf_raw(&self) -> NonNull<LeafNode<T>> {
        match &self.payload.0 {
            DownType::Internal(ptr) => unsafe { ptr.as_ref() }.go_leaf_raw(),
            DownType::Leaf(leaf) => *leaf,
        }
    }
}
