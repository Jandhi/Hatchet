mod pointer;
mod memory;
mod object;
mod flags;
mod structure;

use std::mem::transmute;

use object::{Object, ObjType};

use crate::pointer::Pointer;

fn main() {
    let mut obj = Object::from_string(String::from("value"));

    let borrowed = obj.borrow();
    let copy = obj.copy();

    let ref1 = borrowed.as_string();

    *ref1 = String::from("test");

    let ref2 = copy.as_string();

    *ref2 = String::from("test2");

    println!("{} {} {}", obj, borrowed, copy); 
}