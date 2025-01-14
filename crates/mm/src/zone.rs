use core::{cmp::min, fmt, ptr::NonNull};

use types::linked_list::{self, ListHead};

use crate::page::Page;

pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const MAX_PAGE_ORDER: usize = 10;
pub const _NR_PAGE_ORDER: usize = 10;

pub const MAX_PAGE_CNT: usize = 1 << 12;

pub struct Zone {
    free_area: [linked_list::List<usize>; MAX_PAGE_ORDER],
    pages: [Page; MAX_PAGE_CNT],
    zone_start: usize,
    zone_end: usize,

    _managed_pages: usize,
    _present_pages: usize,
}

pub(crate) fn prev_two_order(num: usize) -> usize {
    usize::BITS as usize - num.leading_zeros() as usize - 1
}

pub(crate) fn _prev_power_of_two(num: usize) -> usize {
    1 << prev_two_order(num)
}

pub fn next_aligned_by(address: usize, alignment: usize) -> usize {
    let remainder = address & (alignment - 1);
    address + (alignment - remainder) * (remainder != 0) as usize
}

pub fn prev_aligned_by(address: usize, alignment: usize) -> usize {
    address & (!alignment + 1)
}

impl Zone {
    pub const fn new() -> Self {
        Zone {
            free_area: [linked_list::List::new(); MAX_PAGE_ORDER],
            pages: [Page::empty(); MAX_PAGE_CNT],
            zone_start: 0,
            zone_end: 0,
            _managed_pages: 0,
            _present_pages: 0,
        }
    }

    pub fn managed_pages(&self) -> usize {
        self._managed_pages
    }

    pub fn present_pages(&self) -> usize {
        self._present_pages
    }

    pub const fn empty() -> Self {
        Self::new()
    }

    fn mark_first_pages_as_allocated(
        &mut self,
        order: usize
    ) -> Option<*mut ListHead<usize>> {
        let result = self.free_area[order].pop_front();
        if let Some(addr) = result {
            let start_page_indx = (addr as usize - self.zone_start) >> PAGE_SHIFT;
            for i in 0..(1 << order) {
                self.pages[start_page_indx + i].inc_use_count();
            }
        }
        result
    }

    fn mark_pages_as_allocated(
        &mut self,
        ptr: *mut u8,
        order: usize
    ) -> Option<*mut ListHead<usize>> {
        let result = self.free_area[order].pop(ptr as *mut ListHead<usize>);
        if let Some(addr) = result {
            let start_page_indx = (addr as usize - self.zone_start) / PAGE_SIZE;
            for i in 0..(1 << order) {
                self.pages[start_page_indx + i].inc_use_count();
            }
        }
        result
    }

    fn mark_pages_as_free(&mut self, ptr: *mut u8, order: usize) {
        self.free_area[order].push_front(ptr as *mut ListHead<usize>);
        let start_page_indx = (ptr as usize - self.zone_start) >> PAGE_SHIFT;
        for i in 0..(1 << order) {
            self.pages[start_page_indx + i].dec_use_count();
            self.pages[start_page_indx + i].set_order(order);
        }
    }

    pub unsafe fn add_to_heap(&mut self, start: usize, end: usize) {
        self.zone_start = next_aligned_by(start, PAGE_SIZE);
        self.zone_end = prev_aligned_by(end, PAGE_SIZE);
        assert!(start < end);

        let mut current_start = start;
        while current_start + PAGE_SIZE <= end {
            let mut order = prev_two_order(end - current_start) - PAGE_SHIFT;
            if order > MAX_PAGE_ORDER - 1 {
                order = MAX_PAGE_ORDER - 1;
            }

            let entry = current_start as *mut ListHead<usize>;
            self.free_area[order].push_front(entry);

            let allocated_pages = 1 << order;
            self._managed_pages += allocated_pages;

            let allocated = allocated_pages << PAGE_SHIFT;
            current_start += allocated;
        }
    }

    pub fn alloc_pages(&mut self, order: usize) -> Result<NonNull<u8>, ()> {
        for i in order..self.free_area.len() {
            if !self.free_area[i].is_empty() {
                let mut block = None;
                for j in (order + 1..i + 1).rev() {
                    block = if block.is_none() {
                        self.mark_first_pages_as_allocated(j)
                    } else {
                        block
                    };
                    match block {
                        Some(block) => {
                            let block_new_size = 1 << (j - 1) << PAGE_SHIFT;

                            let buddy_block = (block as usize + block_new_size) as *mut ListHead<usize>;
                            self.mark_pages_as_free(buddy_block as *mut u8, j - 1);
                        },
                        None => {
                            return Err(());
                        }
                    }
                }

                let alloc_start = match block {
                    Some(block) => Some(block),
                    None => {
                        self.mark_first_pages_as_allocated(order)
                    },
                }.expect("current block should have free space now")
                    as *mut u8;

                let result = NonNull::new(alloc_start);
                if let Some(result) = result {
                    return Ok(result);
                } else {
                    return Err(());
                }
            }
        }
        Err(())
    }

    pub fn alloc_pages_exact(&mut self, size: usize) -> Result<NonNull<u8>, ()> {
        let order = size.next_power_of_two();
        self.alloc_pages(order)
    }

    pub fn free_pages(&mut self, ptr: NonNull<u8>, order: usize) {
        self.mark_pages_as_free(ptr.as_ptr() as *mut u8, order);

        let mut current_ptr = ptr.as_ptr() as usize;
        let mut current_order = order;
        while current_order < self.free_area.len() - 1 {
            let buddy = current_ptr ^ (1 << current_order << PAGE_SHIFT);
            let buddy_first_page_indx = (buddy as usize - self.zone_start) / PAGE_SIZE;

            if self.pages[buddy_first_page_indx].get_use_count() == 0
                && self.pages[buddy_first_page_indx].get_order() == current_order {

                self.mark_pages_as_allocated(buddy as *mut u8, current_order);
                self.mark_pages_as_allocated(current_ptr as *mut u8, current_order);

                current_ptr = min(buddy, current_ptr);
                current_order += 1;

                self.mark_pages_as_free(current_ptr as *mut u8, current_order);

            } else {
                break;
            };
        }
    }

    pub fn free_pages_exact(
        &mut self, 
        ptr: NonNull<u8>, 
        size: usize
    ) {
        let order = size.next_power_of_two();
        self.free_pages(ptr, order)
    }
}

impl fmt::Debug for Zone {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let sizes = &mut [0usize; MAX_PAGE_ORDER];
        for (i, area) in self.free_area.iter().enumerate() {
            sizes[i] = area.count();
        }
        fmt.debug_struct(core::any::type_name::<Self>())
            .field("managed", &self._managed_pages)
            .field("present", &self._present_pages)
            .field("sizes", &sizes)
            .finish()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_align_by() {
        assert_eq!(next_aligned_by(0x1234, 0x1000), 0x2000);
        assert_eq!(prev_aligned_by(0x1234, 0x1000), 0x1000);
    }
}
