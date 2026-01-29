#![allow(unused_variables, dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};

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