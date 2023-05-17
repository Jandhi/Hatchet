use std::{mem::transmute, collections::HashMap};
use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use crate::flags::LOG_OBJECT_LIFECYCLES;

pub fn allocate<T>(item : T) -> *mut T {
    unsafe {
        let layout = Layout::new::<T>();
        let ptr = alloc(layout) as *mut T;
        if ptr.is_null() {
            handle_alloc_error(layout);
        }

        *(ptr) = item;
        ptr
    }
}

pub fn deallocate<T>(ptr : *mut T) {
    unsafe {
        let layout = Layout::new::<T>();
        dealloc(ptr as *mut u8, layout);
    }
}