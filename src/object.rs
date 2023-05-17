use std::{mem::transmute, fmt::Display};

use crate::pointer::Pointer;

type HatchetInteger = i32;
type HatchetString = String;
type HatchetFloat = f32;
type HatchetBool = bool;

#[derive(Debug, Copy, Clone)]
pub enum ObjType {
    I32,
    F32,
    Bool,
    String,
}

pub struct Object {
    my_type : ObjType,
    ptr : Pointer<u64>,
}

impl Object {
    pub fn new<T>(value : T) -> Object where T : Clone, T : Display {
        unsafe { 
            Object { my_type: ObjType::String, ptr: transmute(Pointer::new(value)) }
        }
    }

    pub fn get<T>(&self) -> &mut T where T : Clone, T : Display {
        unsafe {
            let ptr : &Pointer<T> = transmute(&self.ptr);
            ptr.get()
        }
    }

    pub fn copy<T>(&self) -> Object where T : Clone, T : Display {
        Object::new(self.get::<T>().clone())
    }    

    pub fn borrow<T>(&self) -> Object where T : Clone, T : Display {
        unsafe {
            let my_ptr : &Pointer<T> = transmute(&self.ptr);
            let borrowed_ptr = my_ptr.borrow();
            Object { my_type: self.my_type, ptr : transmute(borrowed_ptr)}
        }
    }
}