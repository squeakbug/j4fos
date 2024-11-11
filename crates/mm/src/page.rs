use bitflags::bitflags;

pub const PTE_V: usize = 1 << 0; // valid
bitflags! {
    pub struct PageFlags: usize {
        const V = PTE_V;
    }

}

pub struct Page {
    pub flags: PageFlags,
    pub use_count: usize,
}