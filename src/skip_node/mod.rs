mod linked;
pub mod next;
use std::ptr::NonNull;

use self::next::{Next, NextMode};

pub struct SkipNode<T> {
    pub(crate) payload: T,
    pub(crate) next_ptr: Option<Next<T>>,
}

impl<T> SkipNode<T> {
    pub fn new(payload: T) -> Self {
        let this = Self {
            payload,
            next_ptr: None,
        };
        this
    }

    pub fn brother_num(&self) -> usize {
        match self.next_ptr {
            Some(Next {
                mode: NextMode::Brother,
                ptr,
            }) => {
                let next = unsafe { ptr.as_ref() };
                next.brother_num() + 1
            }
            _ => 1,
        }
    }

    pub fn split_next(&mut self) -> Option<NonNull<SkipNode<T>>> {
        match &mut self.next_ptr {
            Some(next) => {
                next.split();
                Some(next.next())
            }
            None => None,
        }
    }

    pub fn go_next_raw(&self) -> Option<NonNull<SkipNode<T>>> {
        Some(self.next_ptr?.next())
    }

    pub fn go_brother_raw(&self) -> Option<NonNull<SkipNode<T>>> {
        self.next_ptr?.next_brother()
    }
    pub fn go_brother(&self) -> Option<&SkipNode<T>> {
        let next = self.go_brother_raw()?;
        Some(unsafe { next.as_ref() })
    }
}
