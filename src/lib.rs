mod list;

mod node;

pub use list::SkipList;

#[cfg(test)]
mod test {
    use crate::SkipList;

    #[test]
    fn test1() {
        let mut skip_list = SkipList::new();

        skip_list.insert(9);
        skip_list.insert(3);
        skip_list.insert(1i32);
        skip_list.insert(2);
        skip_list.insert(-1);
        skip_list.insert(14i32);
        skip_list.insert(211);
        skip_list.insert(-132);
        println!(
            "len {}, hight {}, w {}",
            skip_list.len(),
            skip_list.hight(),
            skip_list.top_len()
        );

        for i in skip_list.iter() {
            println!("now is {i}")
        }
    }
}
