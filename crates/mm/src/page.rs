use bitflags::bitflags;

pub const PTE_V: usize = 1 << 0; // valid
bitflags! {
    pub struct PageFlags: usize {
        const V = PTE_V;
    }

}

#[derive(Copy, Clone)]
pub struct Page {
    use_count_order: u8
}

impl Page {
    pub const fn empty() -> Self {
        Page {
            use_count_order: 0x0,
        }
    }

    pub fn get_use_count(&self) -> usize {
        (self.use_count_order & 0x3) as usize
    }

    pub fn get_order(&self) -> usize {
        ((self.use_count_order & 0xfc) >> 2) as usize
    }

    pub fn set_use_count(&mut self, use_count: usize) {
        self.use_count_order = (use_count & 0x3) as u8 | (self.use_count_order & 0xfc)
    }

    pub fn inc_use_count(&mut self) {
        let count = self.get_use_count();
        self.set_use_count(count + 1);
    }

    pub fn dec_use_count(&mut self) {
        let count = self.get_use_count();
        self.set_use_count(count - 1);
    }

    pub fn set_order(&mut self, order: usize) {
        self.use_count_order = ((order as u8) & 0x3f) << 2  | (self.use_count_order & 0xfc)
    }
}
