//#![allow(unused_variables dead_code)]

pub mod arena;
pub mod interfaces;
pub mod node;
pub mod results;
use crate::tree::{
    arena::Arena,
    interfaces::{DoublyLinkedNode, NodeLike},
    node::*,
    results::*,
    sealed::{DeadId, Id},
};
mod sealed {

    /// Uniquely identify a `Node`, it is a node handle.
    /// Ids must not be generated randomly on the user-side
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    pub struct Id {
        pub index: usize,
        pub generation: usize,
    }
    impl Id {
        pub fn increase_generation(self) -> Self {
            Self {
                generation: self.generation + 1,
                ..self
            }
        }
    }
    #[derive(Debug)]
    pub struct DeadId(pub Id);

    impl From<Id> for DeadId {
        fn from(value: Id) -> Self {
            DeadId(value)
        }
    }
}

#[derive(Debug)]
pub struct ArenaHandler<T: NodeLike> {
    last_id: usize,
    arena: Arena<T>,
    dead_ids: Vec<DeadId>,
    root_id: Option<Id>,
}

impl<Node: NodeLike> ArenaHandler<Node> {
    pub fn as_mut(&mut self) -> &mut Self {
        self
    }
    pub fn get_mut<'b, 'a: 'b>(&'a mut self, id: Id) -> &'b mut Node {
        self.arena.get_mut_node(id).unwrap()
    }
    pub fn new() -> Self {
        ArenaHandler {
            last_id: 0, // it is used as position in arena
            arena: Arena(Vec::new()),
            dead_ids: Vec::new(),
            root_id: None,
        }
    }
    pub fn set_root_from_none(&mut self, id: Id) -> RootAssignment<Node> {
        if let Some(_root_id) = &self.root_id {
            RootAssignment::<Node>::ExpectedNone
        } else {
            self.root_id = Some(id);
            RootAssignment::<Node>::Succesful
        }
    }
    /// Here from_some means that you want to replace an already set root
    /// in which case the old root is returned as an `OldNode`.
    /// If the root is `None`, a failure variant is returned.
    pub fn set_root_from_some(&mut self, id: Id) -> RootAssignment<Node> {
        if let Some(old_root_id) = self.root_id {
            // take the old node
            let old_root = self
                .arena
                .take_node(old_root_id)
                .expect("If root is not None the Id should be valid");
            RootAssignment::Overridden(OldNode(old_root))
        } else {
            self.root_id = Some(id);
            RootAssignment::ExpectedSome
        }
    }
    pub fn new_node(&mut self, value: Node::Value) -> Id {
        if let Some(dead_id) = self.dead_ids.pop() {
            // means that the position is already allocated
            // only needs to assign a None value to Some(Node)

            let res = self
                .arena
                .assign_from_none(dead_id.0, Node::new(value, dead_id.0)); // need to think more about id dispatching
            match res {
                ArenaAssignmentResult::ExpectedNone => {
                    panic!("Tried to rewrite Some value when not inteded to")
                }
                _ => (),
            }
            dead_id.0
        } else {
            let id = self.gen_id();
            self.arena.push_node(id, Node::new(value, id));
            id
        }
    }
    pub fn gen_id(&mut self) -> Id {
        let id = Id {
            index: self.last_id,
            generation: 0,
        };
        self.last_id += 1;
        id
    }

    pub fn remove_node(&mut self, id: Id) -> Option<Node> {
        self.dead_ids.push(DeadId(id.increase_generation()));
        self.arena.take_node(id)
    }
}


pub trait ConnectNodes{
    type NodeType: NodeLike;

    fn connect(&mut self, id: Id, other_id: Id);
}

impl<T: DoublyLinkedNode> ConnectNodes for ArenaHandler<T>{
    type NodeType = T;

    fn connect(&mut self, id: Id, other_id: Id) {
        self.arena.get_mut_node(id).unwrap().set_next(other_id);
        self.arena.get_mut_node(other_id).unwrap().set_prev(id);
    }
}

/*pub fn connect(&mut self, id: Id, other_id: Id) -> NodeConnectionResult {
    if id == other_id {
        return NodeConnectionResult::SelfConnection;
    }
    match self.arena.get_mut_node(id) {
        Some(node) => match node {
            Some(_) => NodeConnectionResult::AlreadyConnected,
            None => {
                node.next_node_id = Some(other_id);
                NodeConnectionResult::SuccesfullyConnected
            }
        },
        None => NodeConnectionResult::IdNotFound(id),
    }
} */
