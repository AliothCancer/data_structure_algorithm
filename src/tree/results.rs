use crate::tree::{interfaces::NodeLike, node::OldNode, sealed::Id};

pub enum RootAssignment<T: NodeLike>{
    ExpectedNone,
    Succesful,
    Overridden(OldNode<T>),
    ExpectedSome,
}
pub enum ArenaAssignmentResult<T: NodeLike> {
    //InvalidId,      // means position does not exist on arena
    Succesful,      // if the value was None
    Overridden(OldNode<T>),     // if the values was Some but used purposely the overwrite method
    ExpectedNone, // if the values was Some and used the assign_to_none method
    ExpectedSome, // used the overwrite method when not supposed to (on None)
}

/// Represent possible cases when trying to take `Option<Node<T>>` at the given `Id`
pub enum ArenaRemoveResult<T: NodeLike> {
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
