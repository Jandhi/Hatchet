use super::{variables::Variable, function::function::Function};



pub struct Context<'a, 'b> {
    pub variables : Vec<Variable>,
    pub parent_contexts : Vec<&'a Context<'a, 'b>>,
    pub functions : Vec<&'b Function>,
}

pub fn make_empty_context() -> Context<'static, 'static> {
    Context { variables: vec![], parent_contexts: vec![], functions: vec![] }
}

impl Context<'_, '_> {
    pub fn make_child(&self) -> Context {
        let mut parents = self.parent_contexts.clone();
        parents.push(self);
        Context { variables: self.variables.clone(), parent_contexts: parents, functions: self.functions.clone() }
    }
}

impl<'a, 'b> Clone for Context<'a, 'b> {
    fn clone(&self) -> Self {
        Self { variables: self.variables.clone(), parent_contexts: self.parent_contexts.clone(), functions: self.functions.clone() }
    }
}