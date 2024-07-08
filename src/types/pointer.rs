use super::hatchet_type::Type;

#[derive(Debug, Clone)]
pub struct Pointer {
    pub owned : bool,
    pub content : Box<Type>,
}