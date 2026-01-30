use crate::tree::{ArenaHandler, sealed::Id};
// not yet used 
pub trait NodeLike {
    type Value;
    fn new(value: Self::Value, id: Id)-> Self;
    fn get_value(&self) -> &Self::Value;
    fn id(&self) -> Id;
}

pub trait DoublyLinkedNode: NodeLike + Sized{
    fn set_prev(&mut self, id: Id);
    fn set_next(&mut self, id: Id);
    

    fn get_next<'a>(&self, handler: &'a ArenaHandler<Self>) -> Option<&'a Self>;
    fn get_prev<'a>(&self, handler: &'a ArenaHandler<Self>) -> Option<&'a Self>;

    //fn get_mut_next(&mut self, handler: &mut ArenaHandler<Self>) -> Option<&mut Self>;
    //fn get_mut_prev(&mut self, handler: &mut ArenaHandler<Self>) -> Option<&mut Self>;

}
// I'll start with a Binary
pub trait BinaryNode: NodeLike + Sized{
    fn get_left<'a>(&'a self) -> Option<&'a Self>;
    fn get_right<'a>(&'a self) -> Option<&'a Self>;
}


