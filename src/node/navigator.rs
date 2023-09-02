use std::{ptr::NonNull, sync::Arc};

use super::{DownData, SkipNode};

impl<T> SkipNode<T> {
    /// 到达当前节点的最底端节点
    pub fn go_bottom_raw(this: NonNull<Self>) -> NonNull<SkipNode<T>> {
        match unsafe { this.as_ref() }.down {
            DownData::Ptr(ptr) => Self::go_bottom_raw(ptr),
            DownData::Data(_) => this,
        }
    }

    // 获取当前节点的存放的元素
    pub fn get_item(&self) -> Arc<T> {
        match &self.down {
            DownData::Ptr(inner) => unsafe { inner.as_ref() }.get_item(),
            DownData::Data(data) => Arc::clone(data),
        }
    }

    pub(crate) fn get_raw_item(&self) -> &T {
        match &self.down {
            DownData::Ptr(inner) => unsafe { inner.as_ref() }.get_raw_item(),
            DownData::Data(data) => &data,
        }
    }

    /// 到达当前节点下一级，如果为最终节点，返回None
    pub fn go_down(&self) -> Option<&SkipNode<T>> {
        match self.down {
            DownData::Ptr(ptr) => Some(unsafe { ptr.as_ref() }),
            DownData::Data(_) => None,
        }
    }
    pub fn go_down_mut(&mut self) -> Option<&mut SkipNode<T>> {
        match self.down {
            DownData::Ptr(mut ptr) => Some(unsafe { ptr.as_mut() }),
            DownData::Data(_) => None,
        }
    }

    pub fn go_down_raw(&self) -> Option<*mut SkipNode<T>> {
        match self.down {
            DownData::Ptr(ptr) => Some(ptr.as_ptr()),
            DownData::Data(_) => None,
        }
    }

    pub fn go_next(&self) -> Option<&SkipNode<T>> {
        self.next.map(|ptr| unsafe { ptr.as_ref() })
    }
    pub fn go_next_raw(&self) -> Option<*mut SkipNode<T>> {
        self.next.map(|ptr| ptr.as_ptr())
    }
    pub fn go_next_mut(&mut self) -> Option<&mut SkipNode<T>> {
        self.next.map(|mut ptr| unsafe { ptr.as_mut() })
    }
}
