use crate::tree::{
    interfaces::{DoublyLinkedNode, NodeLike},
    sealed::Id,
};

#[derive(Debug)]
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

pub struct OldNode<Node: NodeLike>(pub Node);

impl<T> NodeLike for Node<T> {
    type Value = T;

    fn get_value(&self) -> &Self::Value {
        &self.value
    }

    fn id(&self) -> Id {
        self.id
    }

    fn new(value: Self::Value, id: Id) -> Self {
        Self {
            value,
            id,
            next_node_id: None,
            prev_node_id: None,
        }
    }
}

impl<T> DoublyLinkedNode for Node<T> {
    fn get_next<'a>(&self, handler: &'a super::ArenaHandler<Self>) -> Option<&'a Self> {
        handler.arena.get_node(self.next_node_id.unwrap())
    }

    fn get_prev<'a>(&self, handler: &'a super::ArenaHandler<Self>) -> Option<&'a Self> {
        handler.arena.get_node(self.prev_node_id.unwrap())
    }

    fn set_prev(&mut self, id: Id) {
        self.prev_node_id = Some(id);
    }

    fn set_next(&mut self, id: Id) {
        self.next_node_id = Some(id);
    }
}
