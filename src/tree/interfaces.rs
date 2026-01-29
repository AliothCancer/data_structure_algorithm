use crate::tree::sealed::Id;

pub trait Node {
    fn get_value(&self) -> Option<Id>;
}

pub trait SinglyLinkedNode: Node {
    fn get_next(&self) -> Option<Id>;
}
// I'll start with a Binary
pub trait BinaryNode<N>: Node {
    fn get_left(&self) -> &N;
    fn get_right(&self) -> &N;
}


