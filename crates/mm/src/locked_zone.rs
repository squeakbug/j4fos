use std::{
    alloc::{GlobalAlloc, Layout},
    ops::Deref,
    ptr::NonNull,
};

use spin::Mutex;

use crate::zone::Zone;

pub struct LockedZone(Mutex<Zone>);

impl LockedZone {
    pub const fn new() -> Self {
        LockedZone(Mutex::new(Zone::new()))
    }

    pub const fn empty() -> Self {
        LockedZone(Mutex::new(Zone::new()))
    }
}

impl Deref for LockedZone {
    type Target = Mutex<Zone>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl GlobalAlloc for LockedZone {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
            .lock()
            .alloc(layout)
            .ok()
            .map_or(core::ptr::null_mut(), |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.lock().dealloc(NonNull::new_unchecked(ptr), layout)
    }
}
