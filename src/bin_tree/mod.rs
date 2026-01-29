#![allow(unused_variables, dead_code)]

// FIRST IMPLEMENTATION NO AI
// Actually I just let claude comment my code
// It saw that I have 2 connect implementation, I forgot to
// delete Node::connect after refactoring understanding that
// I was confused and tried to have multiple mutable access
// to the HashMap, for some reason I was trying to cross referencing
// two node, so another kind of connection than the one I was supposed
// to do

use std::collections::{HashMap, VecDeque};

use crate::bin_tree::node::*;

pub mod node;

pub struct LinkedList<T> {
    last_id: u32,
    arena: HashMap<Id, Node<T>>,
    dead_ids: VecDeque<Id>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            last_id: 0,
            arena: HashMap::new(),
            dead_ids: VecDeque::new(),
        }
    }
    pub fn new_node(&mut self, value: T) -> Id {
        let id = self.dead_ids.pop_back().unwrap_or(self.gen_id());
        let node = Node::new(value, id);

        self.arena.insert(id, node);
        id
    }
    pub fn gen_id(&mut self) -> Id {
        self.last_id += 1;
        Id(self.last_id)
    }

    pub fn remove(&mut self, id: Id) {
        self.arena.remove(&id);
        self.dead_ids.push_front(id);
    }

    pub fn connect(&mut self, id: Id, other_id: Id) -> ConnectionResult {
        if id == other_id {
            return ConnectionResult::SelfConnection;
        }
        match self.arena.get_mut(&id){
            Some(node) => {
                match node.next_node_id {
                    Some(_) => {
                        ConnectionResult::AlreadyConnected
                    },
                    None => {
                        node.next_node_id = Some(other_id);
                        ConnectionResult::SuccesfullyConnected
                    },
                }
            
            },
            None => ConnectionResult::IdNotFound(id),
        }
        
    }
}

pub enum ConnectionResult{
    SelfConnection,
    AlreadyConnected,
    SuccesfullyConnected,
    IdNotFound(Id)
}