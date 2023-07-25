use std::rc::Rc;

use crate::my_types::Text;

#[derive(Debug)]
pub enum Assignee {
    Single(Text),
}

