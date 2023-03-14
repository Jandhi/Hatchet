use crate::{state::State};

use super::{strings, math, logic};

pub fn load_stdlib(state : &mut State) {
    math::load(state);
    strings::load(state);
    logic::load(state);
}