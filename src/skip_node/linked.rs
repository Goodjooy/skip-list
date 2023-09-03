use std::ptr::NonNull;

use super::{next::Next, SkipNode};

impl<T> SkipNode<T> {
    pub fn insert_next(&mut self, mut next: NonNull<Self>) {
        let next_value = unsafe { next.as_mut() };
        next_value.next_ptr = self.next_ptr;
        match &mut self.next_ptr {
            Some(ptr) => {
                ptr.ptr = next;
                ptr.as_brother();
            }
            None => self.next_ptr = Some(Next::new(next)),
        }
    }

    pub fn remove_next(&mut self) {
        if let Some(next) = self.next_ptr {
            let next_node = next.next();
            let next_node = unsafe { Box::from_raw(next_node.as_ptr()) };
            self.next_ptr = next_node.next_ptr;
            drop(next_node)
        }
    }
}
