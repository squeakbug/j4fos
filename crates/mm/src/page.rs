use bitflags::bitflags;

pub const PTE_V: usize = 1 << 0; // valid
bitflags! {
    pub struct PageFlags: usize {
        const V = PTE_V;
    }

}

#[derive(Copy, Clone)]
pub struct Page {
    pub use_count: usize,
    pub order: usize,
}