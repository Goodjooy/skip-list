use std::{ops::Deref, ptr::NonNull};

use crate::{leaf_node::LeafNode, skip_node::next::Next};

use super::{DownType, InternalNode};

impl<T: Ord> InternalNode<T> {
    pub(crate) fn check_combine(&mut self) {
        if self.children_num() > 1 {
            if let Some(mut next) = self.next_ptr.and_then(|next| next.next_brother()) {
                let next = unsafe { next.as_mut() };
                next.check_combine()
            }
        } else if let Some(next_ptr) = self.next_ptr.as_ref().and_then(Next::next_brother) {
            let next = unsafe { next_ptr.as_ref() };
            // 当连续2个节点都只有一个孩子，合并
            if next.children_num() == 1 && self.children_num() == 1 {
                self.range_mut().combine(next.range());
                self.child_mut().as_brother();
                self.remove_next();
            }
        }
    }

    pub(crate) fn check_split(&mut self) {
        if self.children_num() > 2 {
            // 孩子部分超过2，需要分裂
            // 在第一个孩子右边分裂
            let next = match self.child_mut() {
                super::DownType::Internal(ptr) => {
                    let node = unsafe { ptr.as_mut().split_next() };
                    match node {
                        Some(ptr) => Some(DownType::Internal(ptr)),
                        None => {
                            // 分裂节点是最后节点，无法分裂
                            None
                        }
                    }
                }
                super::DownType::Leaf(leaf) => {
                    let node = unsafe { leaf.as_mut().split_next() };
                    match node {
                        Some(ptr) => Some(DownType::Leaf(ptr)),
                        None => None,
                    }
                }
            };

            // 这里理论不会出现这个情况，需要分裂但是分裂点无法分裂
            let Some(down) = next else {
                return;
            };

            // 计算新节点范围
            let this_range = match self.payload.0 {
                DownType::Internal(ptr) => unsafe { ptr.as_ref() }.gen_range(),
                DownType::Leaf(leaf) => unsafe { leaf.as_ref() }.gen_range(),
            };
            let next_range = match down {
                DownType::Internal(ptr) => unsafe { ptr.as_ref() }.gen_range(),
                DownType::Leaf(leaf) => unsafe { leaf.as_ref() }.gen_range(),
            };

            *self.range_mut() = this_range;
            let new_node = Box::new(InternalNode::new((down, next_range)));
            self.insert_next(Box::leak(new_node).into());
        }
    }

    /// if return Some if find a Node can insert it
    /// if Return None: need head insert;
    pub(crate) fn find_level_insert(&mut self, data: &T) -> Option<&mut InternalNode<T>> {
        let range = self.range();
        if range.in_range(data) {
            Some(self)
        } else if range.left_range(data) {
            // 小于当前值，能进入这里，说明需要头插
            // 没有在前一节点停止
            None
        } else {
            match self.next_ptr {
                Some(ptr) => {
                    let ret = unsafe { ptr.next().as_mut() }.find_level_insert(data);
                    if let Some(node) = ret {
                        Some(node)
                    } else {
                        Some(self)
                    }
                }
                None => {
                    // 无后继节点，待插入值大于当前节点范围，可以尾插
                    Some(self)
                }
            }
        }
    }

    pub(crate) fn insert(&mut self, data: NonNull<LeafNode<T>>) {
        let insert_node = self.find_level_insert(unsafe { data.as_ref().payload.deref() });
        match insert_node {
            Some(node) => {
                match node.child_mut() {
                    super::DownType::Internal(node) => {
                        // 未到达叶节点，递归第找到叶节点
                        let node = unsafe { node.as_mut() };
                        node.insert(data);
                    }
                    // 到达叶节点
                    super::DownType::Leaf(leaf) => {
                        unsafe { leaf.as_mut().insert(data) };
                    }
                }
                // update range
                node.range_mut()
                    .update(unsafe { data.as_ref().payload.clone() });
                // 插入完毕，检查分裂
                node.check_split()
            }
            // 头插
            None => {
                let data_payload = unsafe { data.as_ref() }.payload.clone();
                match self.child_mut() {
                    // 还未到达叶节点，进入下一级
                    super::DownType::Internal(ptr) => {
                        let node = unsafe { ptr.as_mut() };
                        node.insert(data);
                    }
                    super::DownType::Leaf(leaf) => {
                        // 构造头节点
                        let mut leaf_head = unsafe { Box::from_raw(data.as_ptr()) };
                        let origin_leaf = unsafe { leaf.as_mut() };

                        // 换出原有的内部节点值
                        std::mem::swap(origin_leaf, &mut leaf_head);
                        origin_leaf.next_ptr = Some(Next::new(Box::leak(leaf_head).into()))
                    }
                }
                // update range
                self.range_mut().update(data_payload);
                // 插入完毕，检查分裂条件
                self.check_split();
            }
        }
        // 最后，检查合并条件
        self.check_combine();
    }
}

#[cfg(test)]
mod test {
    use std::{ops::Deref, sync::Arc};

    use crate::{
        internal_node::{DownType, InternalNode, Range},
        leaf_node::LeafNode,
        skip_node::next::Next,
    };

    fn gen_internal_node<T: Ord>(child: T) -> InternalNode<T> {
        let data = Arc::new(child);
        let child = Box::new(LeafNode::new(data.clone()));
        let range = Range::new(data.clone());
        InternalNode::new((DownType::Leaf(Box::leak(child).into()), range))
    }

    #[test]
    fn test_check_split() {
        let mut child = LeafNode::new(Arc::new(1u8));
        let mut child2 = LeafNode::new(Arc::new(2));
        child.insert((&mut child2).into());
        let mut child2 = LeafNode::new(Arc::new(3));
        child.insert((&mut child2).into());

        let range = Range {
            min: Arc::new(1),
            max: Arc::new(3),
        };

        let mut root = InternalNode::new((DownType::Leaf((&mut child).into()), range));
        assert_eq!(root.children_num(), 3);
        assert_eq!(root.brother_num(), 1);
        assert_eq!(root.height(), 1);

        root.check_split();

        assert_eq!(root.brother_num(), 2);
        assert_eq!(root.children_num(), 1);
        assert_eq!(
            root.payload.1,
            Range {
                min: Arc::new(1),
                max: Arc::new(1)
            }
        );
        let brother = unsafe { root.next_ptr.unwrap().next_brother().unwrap().as_ref() };
        assert_eq!(brother.brother_num(), 1);
        assert_eq!(brother.children_num(), 2);
        assert_eq!(
            brother.payload.1,
            Range {
                min: Arc::new(2),
                max: Arc::new(3)
            }
        );
    }
    #[test]
    fn test_check_combine() {
        let mut root1 = gen_internal_node(1);
        let root2 = Box::new(gen_internal_node(5));
        let mut next = Next::new(root2.go_leaf_raw());
        next.split();
        unsafe { root1.go_leaf_raw().as_mut() }.next_ptr = Some(next);
        assert_eq!(root2.brother_num(), 1);
        assert_eq!(root2.children_num(), 1);
        root1.next_ptr = Some(Next::new((Box::leak(root2)).into()));

        assert_eq!(root1.brother_num(), 2);
        assert_eq!(root1.children_num(), 1);

        root1.check_combine();

        assert_eq!(root1.brother_num(), 1);
        assert_eq!(root1.children_num(), 2);
    }

    #[test]
    fn test_head_insert() {
        let mut child = LeafNode::new(Arc::new(2u8));
        let mut child2 = LeafNode::new(Arc::new(1));

        let range = Range::new_in(2, 2);

        let mut root = InternalNode::new((DownType::Leaf((&mut child).into()), range));
        assert_eq!(root.brother_num(), 1);
        assert_eq!(root.children_num(), 1);
        assert_eq!(root.payload.1, Range::new_in(2, 2));

        let insert_node = root.find_level_insert(&1);
        assert!(insert_node.is_none());

        root.insert((&mut child2).into());

        assert_eq!(root.brother_num(), 1);
        assert_eq!(root.children_num(), 2);
        //first child is 1
        assert_eq!(root.go_leaf().payload.deref(), &1);
        assert_eq!(
            unsafe {
                root.go_leaf()
                    .next_ptr
                    .unwrap()
                    .next_brother()
                    .unwrap()
                    .as_ref()
            }
            .payload
            .deref(),
            &2
        );

        assert_eq!(root.payload.1, Range::new_in(1, 2));
    }
    #[test]
    fn test_inner_insert() {
        let mut child = LeafNode::new(Arc::new(2u8));
        let mut child2 = LeafNode::new(Arc::new(5));

        let range = Range::new_in(2, 2);

        let mut root = InternalNode::new((DownType::Leaf((&mut child).into()), range));

        assert_eq!(root.brother_num(), 1);
        assert_eq!(root.children_num(), 1);
        assert_eq!(root.payload.1, Range::new_in(2, 2));

        let insert_node = root.find_level_insert(&5);
        // insert in current node
        assert!(insert_node.is_some());

        root.insert((&mut child2).into());

        assert_eq!(root.brother_num(), 1);
        assert_eq!(root.children_num(), 2);
        //first child is 1
        assert_eq!(root.go_leaf().payload.deref(), &2);
        assert_eq!(
            unsafe {
                root.go_leaf()
                    .next_ptr
                    .unwrap()
                    .next_brother()
                    .unwrap()
                    .as_ref()
            }
            .payload
            .deref(),
            &5
        );

        assert_eq!(root.payload.1, Range::new_in(2, 5));
    }
    #[test]
    fn test_brother_insert() {
        let mut root1 = gen_internal_node(1);
        let mut root2 = gen_internal_node(5);
        root1.next_ptr = Some(Next::new((&mut root2).into()));

        assert_eq!(root1.brother_num(), 2);
        assert_eq!(root1.children_num(), 1);
        assert_eq!(root2.brother_num(), 1);
        assert_eq!(root2.children_num(), 1);

        let data = 15;
        let insert_node = root1.find_level_insert(&data);
        assert!(insert_node.is_some());
        assert_eq!(insert_node.unwrap().range(), &Range::new_in(5, 5));

        let mut child2 = LeafNode::new(Arc::new(data));

        root1.insert((&mut child2).into());

        assert_eq!(root1.brother_num(), 2);
        assert_eq!(root1.children_num(), 1);
        assert_eq!(root2.brother_num(), 1);
        assert_eq!(root2.children_num(), 2);
        assert_eq!(root2.range(), &Range::new_in(5, 15));
    }

    fn new_child(value: u8) -> Box<LeafNode<u8>> {
        Box::new(LeafNode::new(Arc::new(value)))
    }
    #[test]
    fn full_test() {
        let mut root = gen_internal_node(125);

        // add 2
        root.insert(Box::leak(new_child(244)).into());
        root.insert(Box::leak(new_child(100)).into());
        // root -- Node
        //  |       |
        // 100 -.-.125 --244

        // expect
        //1: split to 2
        assert_eq!(root.brother_num(), 2);
        //2: one child,
        assert_eq!(root.children_num(), 1);
        //3: child value is 100
        assert_eq!(root.go_leaf().value(), &100);
        //4: range is 100~100
        assert_eq!(root.range(), &Range::new_in(100, 100));

        // brother1
        let brother = root.go_brother().unwrap();
        //1: brother has 2 child
        assert_eq!(brother.children_num(), 2);
        //2: first child is 125
        assert_eq!(brother.go_leaf().value(), &125);
        //3: second child is 244
        assert_eq!(brother.go_leaf().go_brother().unwrap().value(), &244);
        //4: range is 125~244
        assert_eq!(brother.range(), &Range::new_in(125, 244));

        // add 1 to brother
        root.insert(Box::leak(new_child(255)).into());
        // root -- node -- node
        //  |       |       |
        // 100     125      244 -- 255
        //
        // need combine
        //
        // root  -------- node
        //  |              |
        // 100----- 125   244 -- 255

        // expect
        // root
        // 1: root has 2 brother
        assert_eq!(root.brother_num(), 2);
        // 2: root as 2 child
        assert_eq!(root.children_num(), 2);
        // 3: first child is 100
        assert_eq!(root.go_leaf().value(), &100);
        // 4: second child is 125
        assert_eq!(root.go_leaf().go_brother().unwrap().value(), &125);
        // 5: range is 100~125
        assert_eq!(root.range(), &Range::new_in(100, 125));

        // brother
        let brother = root.go_brother().unwrap();

        //1: brother has 2 children
        assert_eq!(brother.children_num(), 2);
        //2: first child is 244
        assert_eq!(brother.go_leaf().value(), &244);
        //3: second child is 255
        assert_eq!(brother.go_leaf().go_brother().unwrap().value(), &255);
        // 4: range is 244~255
        assert_eq!(brother.range(), &Range::new_in(244, 255));
    }
}
