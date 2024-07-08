use std::fmt::Debug;

use crate::{position::Position, types::hatchet_type::TypeData};

use super::expression::Expression;

#[derive(Debug)]
pub struct Node<TNode : Debug + NodeDisplay> {
    pub content : TNode,
    pub position : Position,
    pub type_data : TypeData,
}

pub trait NodeDisplay {
    fn node_display(&self, depth : usize) -> String;

    fn spacing(depth : usize) -> String {
        "  ".repeat(depth)
    }
}

impl<TNode : Debug + NodeDisplay> NodeDisplay for Node<TNode> {
    fn node_display(&self, depth : usize) -> String {
        self.content.node_display(depth)
    }
}

impl NodeDisplay for String {
    fn node_display(&self, depth : usize) -> String {
        self.clone()
    }
}
