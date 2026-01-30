use crate::tree::{interfaces::{BinaryNode, DoublyLinkedNode, NodeLike}, node::{Node, OldNode}, results::ArenaAssignmentResult, sealed::Id};

#[derive(Debug)]
pub struct Arena<T: NodeLike>(pub Vec<Option<T>>);
impl<Node: NodeLike> Arena<Node> {
    pub fn get_mut_node(&mut self, id : Id) -> Option<&mut Node>{
        if let Some(node) = self.0.get_mut(id.index){
            node.as_mut()
        }else{
            None
        }
    }
    pub fn get_node(&self, id : Id) -> Option<&Node>{
        if let Some(node) = self.0.get(id.index){
            node.as_ref()
        }else{
            None
        }
    }
    /// Take the Option value leaving None at its place
    pub fn take_node(&mut self, id : Id) -> Option<Node>{
        if let Some(node) = self.0.get_mut(id.index){
            node.take()
        }else{
            None
        }
    }
    pub fn push_node(&mut self, id: Id, node: Node,){
        self.allocate_free_space(1);
        self.assign_from_none(id, node);
    }

    fn allocate_free_space(&mut self, number_of_positions: usize) {
        for _ in 0..number_of_positions {
            self.0.push(None);
        }
    }
    /// Will return `AssignmentResult::FailedOccupied`
    /// if the position is not `None` instead of overwriting an already present node
    pub fn assign_from_none(&mut self, id: Id, node: Node) -> ArenaAssignmentResult<Node> {
        if let Some(pos) = self.0.get_mut(id.index) {
            match pos {
                Some(_) => panic!("Expected None found:"),
                None => {
                    let _ = pos.insert(node);
                    ArenaAssignmentResult::Succesful
                }
            }
        } else {
            panic!("Used an invalid Id: {id:?}")
        }
    }
    /// Force overwriting of a Node, return the old value
    /// maybe useful for node swapping(?)
    fn overwrite_some_node(&mut self, id: Id, node: Node) -> ArenaAssignmentResult<Node> {
        if let Some(pos) = self.0.get_mut(id.index) {
            match pos.take() {
                Some(old_node) => {
                    ArenaAssignmentResult::Overridden(OldNode(old_node))
                },
                None => {
                    ArenaAssignmentResult::ExpectedSome
                }
            }
        } else {
            panic!("Used an invalid Id: {id:?}")
        }
    }

    
}

