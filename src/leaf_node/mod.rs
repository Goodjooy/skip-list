use std::{ops::Deref, ptr::NonNull, sync::Arc};

use crate::{internal_node::Range, skip_node::SkipNode};

pub type LeafNode<T> = SkipNode<Arc<T>>;

impl<T> LeafNode<T> {
    pub fn insert(&mut self, data: NonNull<LeafNode<T>>)
    where
        T: Ord,
    {
        let value = unsafe { data.as_ref().payload.deref() };
        let this_value = self.payload.deref();
        let next_value = self.next_ptr.map(|next| unsafe { next.next().as_mut() });
        match next_value {
            Some(next_value) => {
                if value >= this_value && value < next_value.payload.deref() {
                    self.insert_next(data);
                } else {
                    next_value.insert(data);
                }
            }
            None => self.insert_next(data),
        }
    }

    pub fn value(&self) -> &T {
        &self.payload.deref()
    }

    pub fn gen_range(&self) -> Range<T>
    where
        T: Ord,
    {
        let mut init = Range::new(self.payload.clone());
        if let Some(next) = self.go_brother() {
            init.combine(&next.gen_range());
        }
        init
    }
}
