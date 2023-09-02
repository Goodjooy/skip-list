use std::{ptr::NonNull, sync::Arc};

pub(crate) enum DownData<T> {
    Ptr(NonNull<SkipNode<T>>),
    Data(Arc<T>),
}
mod create;
mod modify;
mod navigator;
pub(crate) struct SkipNode<T> {
    pub(crate) next: Option<NonNull<SkipNode<T>>>,
    pub(crate) down: DownData<T>,
}

impl<T> Drop for SkipNode<T> {
    fn drop(&mut self) {
        // todo!()
    }
}

impl<T> SkipNode<T> {
    pub fn level_len(&self) -> usize {
        match self.next {
            Some(ptr) => unsafe { ptr.as_ref() }.level_len() + 1,
            None => 1,
        }
    }

    pub fn skip_next(&self) -> Option<NonNull<SkipNode<T>>> {
        self.go_next()?.next
    }
}
