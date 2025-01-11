use core::{ops::Deref, ptr::NonNull};
use core::alloc::{GlobalAlloc, Layout};
use core::ptr;

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

    pub fn alloc_pages(&self, order: usize) -> *mut u8 {
        self.0
            .lock()
            .alloc_pages(order)
            .ok()
            .map_or(core::ptr::null_mut(), |allocation| allocation.as_ptr())
    }

    pub fn free_pages(&self, ptr: NonNull<u8>, order: usize) {
        self.0
            .lock()
            .free_pages(ptr, order)
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
            .alloc_pages(layout.size().next_power_of_two())
            .map_or(ptr::null_mut(), |p| p.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        match NonNull::new(ptr) {
            Some(nptr) => {
                self.0
                    .lock()
                    .free_pages(nptr, layout.size().next_power_of_two())
            },
            None => { },
        };
        
    }
}
