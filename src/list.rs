use std::{ptr::NonNull, sync::Arc};

use crate::node::SkipNode;

pub mod iter;
mod skip_grow;
pub struct SkipList<T> {
    head: Option<NonNull<SkipNode<T>>>,
    length: usize,
    level: usize,
}

impl<T> SkipList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
            level: 1,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn hight(&self) -> usize {
        self.level
    }

    pub fn top_len(&self) -> usize {
        if let Some(head) = self.head.map(|ptr| unsafe { ptr.as_ref() }) {
            head.level_len()
        } else {
            0
        }
    }
}

impl<T> SkipList<T> {
    pub fn pop_head(&mut self) -> Option<Arc<T>> {
        if let Some(head) = self.head {
            let head = unsafe { Box::from_raw(head.as_ptr()) };
            self.head = head.next;
            self.length -= 1;
            Some(head.get_item())
        } else {
            None
        }
    }

    pub fn insert(&mut self, data: T)
    where
        T: Ord,
    {
        self.length += 1;
        match self.head {
            Some(mut head_ptr) => {
                let head = unsafe { head_ptr.as_mut() };
                let node = SkipNode::new_level(self.level, data);
                match head.find_insert(node) {
                    Ok(_) => (),
                    // head insert
                    Err(mut node) => {
                        let head = unsafe { Box::from_raw(head_ptr.as_ptr()) };
                        // add head
                        node.set_all_next(head);
                        // if node.check_next_redundant() {
                        //     node.remove_next();
                        // }
                        self.head = Some(Box::leak(node).into())
                    }
                }
            }
            // empty list
            None => {
                let node = SkipNode::new_level(self.level, data);
                self.head = Some(Box::leak(node).into())
            }
        };
        self.grow_up();
    }
}

impl<T> Drop for SkipList<T> {
    fn drop(&mut self) {
        let mut level_head = self.head;
        while let Some(level) = level_head {
            level_head = match unsafe { level.as_ref() }.down {
                crate::node::DownData::Ptr(ptr) => Some(ptr),
                crate::node::DownData::Data(_) => None,
            };

            let mut head = Some(level);
            // release this level
            while let Some(free_head) = head {
                let box_head = unsafe { Box::from_raw(free_head.as_ptr()) };
                head = box_head.next;
            }
        }
    }
}
