#![allow(unused_variables, dead_code)]


pub mod node;
pub mod interfaces;
use crate::tree::{node::*, sealed::{DeadId, Id}};

mod sealed {
    /// Uniquely identify a `Node`, it is a node handle.
    /// Ids must not be generated randomly on the user-side
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    pub struct Id(pub usize);

    pub struct DeadId(pub Id);
}


pub struct LinkedList<T> {
    last_id: usize,
    arena: Arena<T>,
    dead_ids: Vec<DeadId>,
    root: Option<Id>
}
struct Arena<T>(Vec<Option<Node<T>>>);
impl<T> Arena<T> {
    fn get_mut_node(&mut self, id : Id) -> Option<&mut Node<T>>{
        if let Some(node) = self.0.get_mut(id.0){
            node.as_mut()
        }else{
            None
        }
    }
    fn get_node(&mut self, id : Id) -> Option<&Node<T>>{
        if let Some(node) = self.0.get(id.0){
            node.as_ref()
        }else{
            None
        }
    }
    fn push_node(&mut self, id: Id, node: Node<T>,){
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
    fn assign_from_none(&mut self, id: Id, node: Node<T>) -> ArenaAssignmentResult<T> {
        if let Some(pos) = self.0.get_mut(id.0) {
            match pos {
                Some(_) => ArenaAssignmentResult::FailedOccupied,
                None => {
                    let _ = pos.insert(node);
                    ArenaAssignmentResult::Succesful
                }
            }
        } else {
            ArenaAssignmentResult::InvalidId
        }
    }
    /// Force overwriting of a Node, return the old value
    /// maybe useful for node swapping(?)
    fn overwrite_some_node(&mut self, id: Id, node: Node<T>) -> ArenaAssignmentResult<T> {
        if let Some(pos) = self.0.get_mut(id.0) {
            match pos.take() {
                Some(old_node) => {
                    ArenaAssignmentResult::Overridden(OldNode::new(old_node))
                },
                None => {
                    ArenaAssignmentResult::ExpectedSome
                }
            }
        } else {
            ArenaAssignmentResult::InvalidId
        }
    }

    /// Take the Option value leaving None at its place
    fn remove_node(&mut self, id: Id) -> ArenaRemoveResult<T> {
        if let Some(n) = self.0.get_mut(id.0) {
            match n.take() {
                Some(old_node) => ArenaRemoveResult::Succesful(OldNode::new(old_node)),
                None => todo!(),
            }
        } else {
            ArenaRemoveResult::InvalidId
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            last_id: 0, // it is used as position in arena
            arena: Arena(Vec::new()),
            dead_ids: Vec::new(),
            root: None,
        }
    }
    pub fn set_root_from_none(&mut self, id: Id)-> RootAssignment<T>{
        if let Some(root) = self.root{
            RootAssignment::Occupied
        }else {
            self.root = Some(id);
            RootAssignment::Succesful
        }
    }
    pub fn set_root_from_some(&mut self, id: Id)-> RootAssignment<T>{
        
        if let Some(root) = self.root{
            // take the old node
            let old_node = self.arena.get_mut_node(root).expect("If root is not None the Id should be valid");
            todo!()
            //RootAssignment::Overridden()
        }else {
            self.root = Some(id);
            RootAssignment::Succesful
        }
    }
    pub fn new_node(&mut self, value: T) -> Id {
        if let Some(dead_id) = self.dead_ids.pop(){
            // means that the position is already allocated
            // only needs to reassign a None value to Some(Node)
            self.arena.assign_from_none(dead_id.0, Node::new(value, dead_id.0)); // need to think more about id dispatching
            dead_id.0
        }else{
            // means I need to first make space on the Vec
            self.arena.allocate_free_space(1);
            // Create a new Id is needed cause it is a new position
            let id = self.gen_id();
            // only then I can assign
            self.arena.assign_from_none(id, Node::new(value, id));
            // or I could have simply used the `Arena::push_node` but this makes it more clear
            id
        }
    }
    pub fn gen_id(&mut self) -> Id {
        self.last_id += 1;
        Id(self.last_id)
    }

    pub fn remove(&mut self, id: Id) -> ArenaRemoveResult<T> {
        self.dead_ids.push(DeadId(id));
        self.arena.remove_node(id)
        
    }

    pub fn connect(&mut self, id: Id, other_id: Id) -> NodeConnectionResult {
        if id == other_id {
            return NodeConnectionResult::SelfConnection;
        }
        match self.arena.get_mut_node(id) {
            Some(node) => match node.next_node_id {
                Some(_) => NodeConnectionResult::AlreadyConnected,
                None => {
                    node.next_node_id = Some(other_id);
                    NodeConnectionResult::SuccesfullyConnected
                }
            },
            None => NodeConnectionResult::IdNotFound(id),
        }
    }
}


pub enum RootAssignment<T>{
    Occupied,
    Succesful,
    Overridden(OldNode<T>),
    ExpectedSome,
}
pub enum ArenaAssignmentResult<T> {
    InvalidId,      // means position does not exist on arena
    Succesful,      // if the value was None
    Overridden(OldNode<T>),     // if the values was Some but used purposely the overwrite method
    FailedOccupied, // if the values was Some and used the assign_to_none method
    ExpectedSome, // used the overwrite method when not supposed to (on None)
}

/// Represent possible cases when trying to take `Option<Node<T>>` at the given `Id`
pub enum ArenaRemoveResult<T> {
    InvalidId, // position in the `arena` is not valid
    Succesful(OldNode<T>),
    TriedToRemoveNone, // happen when removing a node which is not present
}
pub enum NodeConnectionResult {
    SelfConnection,
    AlreadyConnected,
    SuccesfullyConnected,
    IdNotFound(Id),
}
