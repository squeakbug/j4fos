use core::{fmt, ptr};

#[derive(Copy, Clone)]
pub struct List {
    head: *mut usize,
}

// TODO: minimum size of object must be specified by type system
// This is intrusive list, so it's worth considering
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ListHead {
    next: *mut ListHead,
    prev: *mut ListHead,
}

impl List {
    pub const fn new() -> List {
        List {
            head: ptr::null_mut(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    pub fn push_front(&mut self, item: *mut usize) { 
        let litem = item as *mut ListHead;      

        unsafe {
            if self.head.is_null() {
                self.head = item;
                (*litem).next = litem;
                (*litem).prev = litem;
            } else {
                let lhead = self.head as *mut ListHead;
                (*litem).next = lhead;
                (*litem).prev = (*lhead).prev;

                let lprev = (*lhead).prev as *mut ListHead;
                (*lhead).prev = litem;
                (*lprev).next = litem;
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<*mut usize> {
        match self.is_empty() {
            true => None,
            false => unsafe {
                let lhead = self.head as *mut ListHead;
                let litem = (*lhead).prev as *mut ListHead;
                let lnew_prev = (*litem).prev as *mut ListHead;

                (*lnew_prev).next = lhead;
                (*lhead).prev = lnew_prev;
                if self.head == litem as *mut usize {
                    self.head = ptr::null_mut();
                }

                (*litem).next = ptr::null_mut();
                (*litem).prev = ptr::null_mut();
                Some(litem as *mut usize)
            }
        }
    }

    pub fn pop(&mut self, list_head: *mut usize) -> Option<*mut usize> {
        match self.is_empty() {
            true => None,
            false => unsafe {
                let lhead = list_head as *mut ListHead;
                let lnext = (*lhead).next;
                let lprev = (*lhead).prev;
                (*lnext).prev = lprev;
                (*lprev).next = lnext;

                if self.head == lhead as *mut usize {
                    self.head = (*lhead).next as *mut usize;
                }

                Some(list_head)
            }
        }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            head: self.head,
            passed: false,
            curr: self.head,
        }
    }
}

unsafe impl Send for List {}

impl fmt::Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

pub struct Iter {
    head: *mut usize,
    passed: bool,
    curr: *mut usize,
}

impl Iterator for Iter {
    type Item = *mut usize;

    fn next(&mut self) -> Option<Self::Item> {
        let litem = self.curr as *const ListHead;
        if litem == self.head as *const ListHead && self.passed {
            None
        } else {
            self.passed = true;
            self.curr = unsafe { (*litem).next } as *mut usize;
            Some(litem as *mut usize)
        }
    }
}

// TODO: impl Rev<'a> for Iter<'a> { }
// TODO: impl Rev<'a> for IterMut<'a> { }
