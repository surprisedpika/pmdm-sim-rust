use std::mem;
use std::ptr;
use crate::mem::*;

pub trait Updatable {
    // Update self from memory
    fn update(&mut self, memory: &Memory, this: &Pointer<Self>) where Self: Sized { unsafe {
        ptr::copy_nonoverlapping(Box::into_raw(
            this.read(memory).unwrap()
        ), ptr::from_mut(self), mem::size_of::<Self>());
    } }
}

pub trait Constructor {
    fn ctor(&mut self, memory: &mut Memory, this: Pointer<Self>) where Self: Sized;
}
