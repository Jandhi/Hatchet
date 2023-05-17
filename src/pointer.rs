use std::{mem::transmute, fmt::Display, ops::{Deref, DerefMut}};

use crate::{memory::{allocate, deallocate}, flags::LOG_OBJECT_LIFECYCLES};

pub struct Pointer<T> where T : Display {
    ptr : *mut T,
    is_owner : bool
}

impl<T> Drop for Pointer<T> where T : Display {
    fn drop(&mut self) {
        if self.is_owner
        {
            self.free();
        }
    }
}

impl<T> Deref for Pointer<T> where T : Display {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> DerefMut for Pointer<T> where T : Display {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get()
    }
}

impl<T> Display for Pointer<T> where T : Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ptr({})", self.get())
    }
}

impl<T> Pointer<T> where T : Display {
    pub fn new(item : T) -> Pointer<T> {
        Pointer { ptr: allocate(item), is_owner : true }
    }

    pub fn borrow(&self) -> Pointer<T> {
        Pointer { ptr: self.ptr, is_owner: false }
    }

    pub fn free(&mut self) {
        if !self.is_owner {
            panic!("You can't free data that you don't own!");
        }

        unsafe {
            if LOG_OBJECT_LIFECYCLES {
                let raw : u64 = transmute(self.ptr);
                println!("Freeing ptr at {} with value {}", raw, self.get());
            }
        }

        deallocate(self.ptr);
    }

    pub fn get(&self) -> &mut T {
        unsafe {
            return transmute(self.ptr)
        }
    }
}
