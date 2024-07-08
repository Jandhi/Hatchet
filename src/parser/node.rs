use std::rc::Rc;

use crate::{position::Position, typechecker::hatchet_type::HatchetType};

pub struct Node {
    pub position : Position,
    pub children : Vec<Rc<Node>>,
    pub hatchet_type : HatchetType
}