use std::ptr::NonNull;

use crate::{node::SkipNode, SkipList};

impl<T> SkipList<T> {
    pub(super) fn grow_up(&mut self) {
        let Some(head_ptr) = self.head else {
            return;
        };

        let head = unsafe { head_ptr.as_ref() };
        let len = head.level_len();
        if len > 2 {
            let new_level = SkipNode::new_skip(head_ptr);
            let mut ptr = NonNull::from(Box::leak(new_level));
            self.head = Some(ptr);

            let mut now_head = unsafe { ptr.as_mut() };
            let mut old_head = head_ptr;
            while let Some(node) = unsafe { old_head.as_ref().skip_next() } {
                old_head = node;
                let new_level = SkipNode::new_skip(node);
                now_head = now_head.set_next(new_level);
            }
            self.level += 1;
        }
    }

    pub(super) fn cut_off(&mut self) {
        todo!()
    }
}
