use std::{mem::transmute, fmt::Display};

use crate::pointer::Pointer;

type HatchetInteger = i32;
type HatchetString = String;
type HatchetFloat = f32;
type HatchetBool = bool;
type HatchetStruct = crate::structure::Struct;


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

impl Drop for Object {
    fn drop(&mut self) {
        self.free();
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.my_type {
            ObjType::I32 => write!(f, "{}", self.get::<HatchetInteger>()),
            ObjType::F32 => write!(f, "{}", self.get::<HatchetFloat>()),
            ObjType::Bool => write!(f, "{}", self.get::<HatchetBool>()),
            ObjType::String => write!(f, "{}", self.get::<HatchetString>()),
        }
    }
}

impl Object {
    
    // Initializatiom

    pub fn from_string(value : String) -> Object {
        unsafe { 
            Object { my_type: ObjType::String, ptr: transmute(Pointer::new(value)) }
        }
    }

    pub fn new<T>(value : T, my_type : ObjType) -> Object where T : Clone, T : Display {
        unsafe { 
            Object { my_type: my_type, ptr: transmute(Pointer::new(value)) }
        }
    }

    // Copying

    pub fn copy(&self) -> Object {
        match self.my_type {
            ObjType::I32 => self.priv_copy::<HatchetInteger>(),
            ObjType::F32 => self.priv_copy::<HatchetFloat>(),
            ObjType::Bool => self.priv_copy::<HatchetBool>(),
            ObjType::String => self.priv_copy::<HatchetString>(),
        }
    }

    fn priv_copy<T>(&self) -> Object where T : Clone, T : Display {
        Object::new(self.get::<T>().clone(), self.my_type)
    }    

    pub fn borrow(&self) -> Object {
        match self.my_type {
            ObjType::I32 => self.priv_borrow::<HatchetInteger>(),
            ObjType::F32 => self.priv_borrow::<HatchetFloat>(),
            ObjType::Bool => self.priv_borrow::<HatchetBool>(),
            ObjType::String => self.priv_borrow::<HatchetString>(),
        }
    }

    fn priv_borrow<T>(&self) -> Object where T : Clone, T : Display {
        unsafe {
            let borrowed_ptr = self.cast_ptr::<T>().borrow();
            Object { my_type: self.my_type, ptr : transmute(borrowed_ptr)}
        }
    }

    // Data Access

    pub fn get<T>(&self) -> &mut T where T : Clone, T : Display {
        self.cast_ptr::<T>().get()
    }

    fn cast_ptr<T>(&self) -> &Pointer<T> where T : Clone, T : Display {
        unsafe {
            transmute(&self.ptr)
        }
    }

    pub fn as_int(&self) -> &mut HatchetInteger {
        self.get()
    }

    pub fn as_float(&self) -> &mut HatchetFloat {
        self.get()
    }

    pub fn as_bool(&self) -> &mut HatchetBool {
        self.get()
    }

    pub fn as_string(&self) -> &mut HatchetString {
        self.get()
    }

    // Destructor

    pub fn free(&mut self) {
        if !self.ptr.is_owner {
            return;
        }

        unsafe {
            match self.my_type {
                ObjType::I32 =>    transmute::<&mut Pointer<u64>, &mut Pointer<HatchetInteger>>(&mut self.ptr).free(),
                ObjType::F32 =>    transmute::<&mut Pointer<u64>, &mut Pointer<HatchetFloat  >>(&mut self.ptr).free(),
                ObjType::Bool =>   transmute::<&mut Pointer<u64>, &mut Pointer<HatchetBool   >>(&mut self.ptr).free(),
                ObjType::String => transmute::<&mut Pointer<u64>, &mut Pointer<HatchetString >>(&mut self.ptr).free(),
            }
        }
    }
}