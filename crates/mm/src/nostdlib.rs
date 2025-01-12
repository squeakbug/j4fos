use crate::layout::PHYSTOP;
use crate::zone::{next_aligned_by, PAGE_SIZE};
use crate::locked_zone::LockedZone;

unsafe extern "C" {
    // first address after kernel, defined by kernel.ld
    unsafe static mut end: [u8; 0];
}

#[global_allocator]
pub static KMEM: LockedZone = LockedZone::new();

pub fn init() {
    unsafe {
        KMEM.lock().add_to_heap(
            next_aligned_by(end.as_ptr() as usize, PAGE_SIZE), 
            next_aligned_by(PHYSTOP, PAGE_SIZE)
        )
    }
}
