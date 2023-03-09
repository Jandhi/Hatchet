use std::collections::HashMap;

use crate::{scope::Scope, state::State};

use super::{strings, math, logic};

pub fn load_stdlib(state : &mut State) {
    math::load(state);
    strings::load(state);
    logic::load(state);
}