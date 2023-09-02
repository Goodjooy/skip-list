use std::{ptr::NonNull, sync::Arc};

use super::{DownData, SkipNode};

impl<T> SkipNode<T> {
    pub fn new_normal(data: T) -> Box<Self> {
        Box::new(Self {
            next: None,
            down: DownData::Data(Arc::new(data)),
        })
    }

    pub fn new_skip(down: NonNull<SkipNode<T>>) -> Box<Self> {
        Box::new(Self {
            next: None,
            down: DownData::Ptr(down),
        })
    }

    pub fn new_level(level: usize, data: T) -> Box<SkipNode<T>> {
        match level {
            1 => Self::new_normal(data),
            l => {
                let down = Self::new_level(l - 1, data);
                Self::new_skip(Box::leak(down).into())
            }
        }
    }
}
