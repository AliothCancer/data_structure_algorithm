use crate::tree::sealed::Id;


pub struct Node<T> {
    pub value: T,
    pub id: Id,
    pub next_node_id: Option<Id>,
    pub prev_node_id: Option<Id>,
}

impl<T> Node<T> {
    pub fn new(value: T, id: Id) -> Self {
        Self {
            value,
            id,
            next_node_id: None,
            prev_node_id: None,
        }
    }
}

pub struct OldNode<T>(Node<T>);
impl<T> OldNode<T> {
    pub fn new(node: Node<T>) -> Self {
        Self(node)
    }
}
