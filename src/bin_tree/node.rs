pub struct Node<T> {
    pub value: T,
    pub id: Id,
    pub next_node_id: Option<Id>,
}

impl<T> Node<T> {
    pub fn new(value: T, id: Id) -> Self {
        Self {
            value,
            id,
            next_node_id: None,
        }
    }

    pub fn connect(&mut self, other_node: &mut Node<T>) {
        self.next_node_id = Some(other_node.id);
        other_node.next_node_id = Some(self.id);
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Id(pub u32);
