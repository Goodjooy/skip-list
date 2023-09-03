use std::ptr::NonNull;

use super::SkipNode;

#[derive(Debug, Clone, Copy)]
pub enum NextMode {
    Brother,
    OuterBrother,
}
#[derive(Debug)]
pub struct Next<T> {
    pub(super) mode: NextMode,
    pub ptr: NonNull<SkipNode<T>>,
}

impl<T> Copy for Next<T> {}

impl<T> Clone for Next<T> {
    fn clone(&self) -> Self {
        Self {
            mode: self.mode.clone(),
            ptr: self.ptr.clone(),
        }
    }
}

impl<T> Next<T> {
    pub fn new(ptr: NonNull<SkipNode<T>>) -> Self {
        Self {
            mode: NextMode::Brother,
            ptr,
        }
    }

    pub fn next_brother(&self) -> Option<NonNull<SkipNode<T>>> {
        match self.mode {
            NextMode::Brother => Some(self.ptr),
            NextMode::OuterBrother => None,
        }
    }

    pub fn next(&self) -> NonNull<SkipNode<T>> {
        self.ptr
    }

    pub fn as_brother(&mut self) {
        self.mode = NextMode::Brother
    }

    pub fn split(&mut self) {
        self.mode = NextMode::OuterBrother
    }
}
