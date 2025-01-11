#![no_main]
#![no_std]

pub mod page;
pub mod zone;
pub mod layout;

use layout::PHYSTOP;
pub use zone::Zone;

pub mod locked_zone;
pub use locked_zone::LockedZone;
use zone::{next_aligned_by, PAGE_SIZE};

#[global_allocator]
pub static KMEM: LockedZone = LockedZone::new();

unsafe extern "C" {
    // first address after kernel, defined by kernel.ld
    unsafe static mut end: [u8; 0];
}

pub fn init() {
    unsafe {
        KMEM.lock().add_to_heap(
            next_aligned_by(end.as_ptr() as usize, PAGE_SIZE), 
            next_aligned_by(PHYSTOP, PAGE_SIZE)
        )
    }
}
