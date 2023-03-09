use crate::scope::Scope;

pub struct State {
    pub scopes : Vec<Scope>
}

impl State {
    pub fn contains_operator(&self, name : &String) -> bool {
        for scope in &self.scopes {
            if scope.operators.contains_key(name) {
                return  true;
            }
        }

        return  false;
    }
}