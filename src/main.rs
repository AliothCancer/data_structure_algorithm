use crate::tree::{ArenaHandler, ConnectNodes, node::Node};

mod tree;
fn main() {
    let mut handler = ArenaHandler::<Node<i32>>::new();

    let root = handler.new_node(45);
    handler.set_root_from_none(root);
    let ch1 = handler.new_node(44);
    let ch2 = handler.new_node(49);
    let ch3 = handler.new_node(90);
    handler.remove_node(ch3);
    
    dbg!(&handler);
    handler.connect(root, ch1);
    dbg!(&handler);

}
