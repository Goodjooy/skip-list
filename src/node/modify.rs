use super::SkipNode;

impl<T> SkipNode<T> {
    pub fn set_next(&mut self, rhs: Box<Self>) -> &mut Self {
        assert!(self.next.is_none(), "insert next to none Null next");
        let mut ptr = Box::leak(rhs).into();
        self.next = Some(ptr);

        unsafe { ptr.as_mut() }
    }

    pub fn insert_next(&mut self, rhs: Box<Self>) {
        let rhs = Box::leak(rhs);
        rhs.next = self.next;
        self.next = Some(rhs.into())
    }

    pub fn set_all_next(&mut self, rhs: Box<Self>) {
        if let (Some(this), Some(rhs)) = (self.go_down_mut(), rhs.go_down_raw()) {
            let rhs = unsafe { Box::from_raw(rhs) };
            this.set_all_next(rhs);
        }
        self.set_next(rhs);
    }

    pub fn remove_next(&mut self) {
        if let Some(ptr) = self.next {
            let node = unsafe { Box::from_raw(ptr.as_ptr()) };
            self.next = node.next;

            drop(node);
        }
    }

    /// 检查下一个节点是否为冗余的
    pub fn check_next_redundant(&self) -> bool {
        // 最底节点，不处理
        let Some(down) = self.go_down() else {
            return false;
        };

        // self 前进2跳
        let Some(this) = self
            .go_next()
            .and_then(|node| node.go_next())
            .and_then(|node| node.go_down_raw())
            .map(|down| down as usize)
        else {
            return false;
        };

        // down 前进2 条
        let Some(down) = down
            .go_next()
            .and_then(|node| node.go_next_raw())
            .map(|ptr| ptr as usize)
        else {
            return false;
        };
        println!("this: {this}, down: {down}");

        this == down
    }
}

impl<T: Ord> SkipNode<T> {
    /// find a node can insert data behind it
    /// ## Note
    /// in current Level
    ///
    /// ## Return
    /// if return None, not found a node fit,should insert to head
    /// if return Some, can insert behind return Node
    pub fn find_insert_node(&mut self, data: &T) -> Option<&mut SkipNode<T>> {
        let this_item = self.get_raw_item();

        match self.next {
            // 有后续节点
            Some(mut ptr) => {
                let node = unsafe { ptr.as_mut() };
                // 如果待插入值大于当前节点值
                if data > this_item {
                    match node.find_insert_node(data) {
                        // 可以在后续节点插入
                        ret @ Some(_) => ret,
                        // 不可在后续节点插入，在当前节点后car
                        None => Some(self),
                    }
                } else {
                    // 待插入值小于当前节点，不可在当前节点后插入
                    None
                }
            }
            None => {
                // 无后续节点
                // 带插入值大于自身，返回Self
                if data > this_item {
                    Some(self)
                } else {
                    //否则返回None, 表示不可插入
                    None
                }
            }
        }
    }

    /// auto insert the node to the tree,
    ///
    /// return Err(rhs) means need head insert
    pub fn find_insert<'node>(&mut self, rhs: Box<SkipNode<T>>) -> Result<(), Box<SkipNode<T>>> {
        let data = rhs.get_raw_item();
        let node = self.find_insert_node(data);
        match node {
            Some(node) => {
                // get down,
                // if None, reach bottom
                if let (Some(down_node), Some(insert_down)) =
                    (node.go_down_mut(), rhs.go_down_raw())
                {
                    let insert_down = unsafe { Box::from_raw(insert_down) };
                    down_node.find_insert(insert_down)?;
                }
                // insert current level to the node
                node.insert_next(rhs);
                // if node.check_next_redundant() {
                //     node.remove_next();
                // }
                Ok(())
            }
            None => Err(rhs),
        }
    }
}
