use crate::tree::{LinkedList};

mod tree;
fn main() {
    let mut list = LinkedList::new();

    let root = list.new_node(45);
    let ch1 = list.new_node(44);
    let ch2 = list.new_node(49);

    list.connect(root, ch1);

    list.connect(ch1, ch2);
    
}
