use std::{
    alloc::Layout,
    cmp::{min, max}, fmt,
    mem::size_of,
    ptr::NonNull,
};

use types::linked_list;

const MAX_PAGE_ORDER: usize = 10;
const _NR_PAGE_ORDER: usize = 10;

pub struct Zone {
    free_area: [linked_list::List; MAX_PAGE_ORDER],

    _managed_pages: usize,
    _present_pages: usize,
}

pub(crate) fn prev_power_of_two(num: usize) -> usize {
    1 << (usize::BITS as usize - num.leading_zeros() as usize - 1)
}

impl Zone {
    pub const fn new() -> Self {
        Zone {
            free_area: [linked_list::List::new(); MAX_PAGE_ORDER],
            _managed_pages: 0,
            _present_pages: 0,
        }
    }

    pub const fn empty() -> Self {
        Self::new()
    }

    pub unsafe fn add_to_heap(&mut self, mut start: usize, mut end: usize) {
        start = (start + size_of::<usize>() - 1) & (!size_of::<usize>() + 1);
        end &= !size_of::<usize>() + 1;
        assert!(start <= end);

        let mut current_start = start;

        while current_start + size_of::<usize>() <= end {
            let lowbit = current_start & (!current_start + 1);
            let mut size = min(lowbit, prev_power_of_two(end - current_start));

            let mut order = size.trailing_zeros() as usize;
            if order > MAX_PAGE_ORDER - 1 {
                order = MAX_PAGE_ORDER - 1;
                size = 1 << order;
            }

            self.free_area[order].push_front(current_start as *mut usize);
            current_start += size;
        }
    }

    pub unsafe fn init(&mut self, start: usize, size: usize) {
        self.add_to_heap(start, start + size);
    }

    pub fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, ()> {
        let size = max(
            layout.size().next_power_of_two(),
            max(layout.align(), size_of::<usize>()),
        );
        let class = size.trailing_zeros() as usize;
        for i in class..self.free_area.len() {
            if !self.free_area[i].is_empty() {
                for j in (class + 1..i + 1).rev() {
                    if let Some(block) = self.free_area[j].pop_front() {
                        let block_new_size = 1 << (j - 1);
                        self.free_area[j - 1]
                            .push_front((block as usize + block_new_size) as *mut usize);
                        self.free_area[j - 1].push_front(block);
                    } else {
                        return Err(());
                    }
                }

                let result = NonNull::new(
                    self.free_area[class]
                        .pop_front()
                        .expect("current block should have free space now")
                        as *mut u8,
                );
                if let Some(result) = result {
                    return Ok(result);
                } else {
                    return Err(());
                }
            }
        }
        Err(())
    }

    pub fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout) {
        let size = max(
            layout.size().next_power_of_two(),
            max(layout.align(), size_of::<usize>()),
        );
        let class = size.trailing_zeros() as usize;

        self.free_area[class].push_front(ptr.as_ptr() as *mut usize);

        let mut current_ptr = ptr.as_ptr() as usize;
        let mut current_class = class;

        while current_class < self.free_area.len() - 1 {
            let buddy = current_ptr ^ (1 << current_class);
            let mut buddy2 = None;
            for block in self.free_area[current_class].iter() {
                if block as usize == buddy {
                    buddy2 = Some(block);
                    break;
                }
            }

            if let Some(buddy2) = buddy2 {
                self.free_area[current_class].pop(buddy2);
                self.free_area[current_class].pop_front();
                current_ptr = min(current_ptr, buddy);
                current_class += 1;
                self.free_area[current_class].push_front(current_ptr as *mut usize);
            } else {
                break;
            }
        }
    }
}

impl fmt::Debug for Zone {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<Self>())
            .finish()
    }
}