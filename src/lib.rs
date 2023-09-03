mod internal_node;
mod leaf_node;
pub mod skip_list;
mod skip_node;

#[cfg(test)]
mod test {
    use rand::Rng;

    #[test]
    fn test2() {
        use crate::skip_list::SkipLinkedList;
        use rand;
        let mut list = SkipLinkedList::new();
        let mut rand = rand::thread_rng();
        for _ in 0..10000 {
            list.insert(rand.gen_range(i32::MIN..=i32::MAX))
        }

        for i in list.iter() {
            println!("now is {i}")
        }
        println!("height {}, len {}", list.height(), list.len());
    }
}
