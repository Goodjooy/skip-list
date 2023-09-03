pub mod iter;
use std::{ptr::NonNull, sync::Arc};

use crate::{
    internal_node::{DownType, InternalNode, Range},
    leaf_node::LeafNode,
};

pub struct SkipLinkedList<T> {
    head: Option<NonNull<InternalNode<T>>>,
    length: usize,
    height: usize,
}

impl<T> Drop for SkipLinkedList<T> {
    fn drop(&mut self) {
        if let Some(head) = self.head {
            let mut head = unsafe { Box::from_raw(head.as_ptr()) };
            let mut next_ptr = head.go_next_raw();

            while let Some(next) = next_ptr {
                let node = unsafe { Box::from_raw(next.as_ptr()) };
                next_ptr = node.go_next_raw();

                drop(node)
            }
            loop {
                match head.child() {
                    DownType::Internal(ptr) => {
                        head = unsafe { Box::from_raw(ptr.as_ptr()) };
                        let mut next_ptr = head.go_next_raw();

                        while let Some(next) = next_ptr {
                            let node = unsafe { Box::from_raw(next.as_ptr()) };
                            next_ptr = node.go_next_raw();

                            drop(node)
                        }
                    }
                    DownType::Leaf(ptr) => {
                        let first = unsafe { Box::from_raw(ptr.as_ptr()) };
                        let mut next_ptr = first.go_next_raw();

                        while let Some(next) = next_ptr {
                            let node = unsafe { Box::from_raw(next.as_ptr()) };
                            next_ptr = node.go_next_raw();
                            drop(node)
                        }
                        break;
                    }
                }
            }
        }
    }
}

impl<T> SkipLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
            height: 0,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn insert(&mut self, data: T)
    where
        T: Ord,
    {
        let skip_node = Box::new(LeafNode::new(Arc::new(data)));
        match self.head {
            Some(mut head) => unsafe { head.as_mut() }.insert(Box::leak(skip_node).into()),
            None => {
                let range = Range::new(skip_node.payload.clone());
                let head = Box::new(InternalNode::new((
                    DownType::Leaf(Box::leak(skip_node).into()),
                    range,
                )));
                self.head = Some(Box::leak(head).into());
                self.height += 1;
            }
        }
        self.length += 1;
        // check combine
        // create temp node
        if let Some(head) = self.head {
            let range = unsafe { head.as_ref() }.gen_range();
            let mut temp = Box::new(InternalNode::new((DownType::Internal(head), range)));
            temp.check_split();
            temp.check_combine();

            // 如果出现next ,需要增加高度
            if temp.brother_num() > 1 {
                self.head = Some(Box::leak(temp).into());
                self.height += 1;
            }
        }
    }
}
