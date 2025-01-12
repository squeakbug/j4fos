use core::{
    {fmt, ptr},
    marker::PhantomData,
};

#[derive(Copy, Clone)]
pub struct List<T> {
    head: *mut ListHead<T>,
    phantom: PhantomData<T>
}

// TODO: minimum size of object must be specified by type system
// This is intrusive list, so it's worth considering
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ListHead<T> {
    pub next: *mut ListHead<T>,
    pub prev: *mut ListHead<T>,
    pub data: T,
}

impl<T> fmt::Debug for ListHead<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(core::any::type_name::<Self>())
            .field("next", &self.next)
            .field("prev", &self.prev)
            .finish()
    }
}

impl<T> List<T> {
    pub const fn new() -> List<T> {
        List {
            head: ptr::null_mut(),
            phantom: PhantomData,
        }
    }

    pub fn count(&self) -> usize {
        self.iter().count()
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    pub fn push_front(&mut self, item: *mut ListHead<T>) {
        let litem = item as *mut ListHead<T>;

        unsafe {
            if self.head.is_null() {
                self.head = item;
                (*litem).next = litem;
                (*litem).prev = litem;
            } else {
                let lhead = self.head as *mut ListHead<T>;
                (*litem).next = lhead;
                (*litem).prev = (*lhead).prev;

                let lprev = (*lhead).prev as *mut ListHead<T>;
                (*lhead).prev = litem;
                (*lprev).next = litem;
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<*mut ListHead<T>> {
        match self.is_empty() {
            true => None,
            false => unsafe {
                let lhead = self.head as *mut ListHead<T>;
                let litem = (*lhead).prev as *mut ListHead<T>;

                if litem == lhead {
                    (*lhead).next = ptr::null_mut();
                    (*lhead).prev = ptr::null_mut();
                    self.head = ptr::null_mut();
                    Some(lhead)
                } else {
                    let lnew_prev = (*litem).prev as *mut ListHead<T>;
                    (*lnew_prev).next = lhead;
                    (*lhead).prev = lnew_prev;
                    if self.head == litem {
                        self.head = ptr::null_mut();
                    }

                    (*litem).next = ptr::null_mut();
                    (*litem).prev = ptr::null_mut();
                    Some(litem)
                }
            }
        }
    }

    pub fn pop(&mut self, list_head: *mut ListHead<T>) -> Option<*mut ListHead<T>> {
        match self.is_empty() {
            true => None,
            false => unsafe {
                let lhead = list_head as *mut ListHead<T>;
                let lnext = (*lhead).next;
                let lprev = (*lhead).prev;
                (*lnext).prev = lprev;
                (*lprev).next = lnext;

                if self.head == lhead{
                    self.head = if (*lhead).next == lhead {
                        ptr::null_mut()
                    } else { 
                        (*lhead).next
                    }
                }

                Some(list_head)
            }
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            head: self.head,
            passed: false,
            curr: self.head,
            phantom: PhantomData,
        }
    }
}

unsafe impl<T> Send for List<T> {}

impl<T> fmt::Debug for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

pub struct Iter<T> {
    head: *mut ListHead<T>,
    passed: bool,
    curr: *mut ListHead<T>,
    phantom: PhantomData<T>,
}

impl<T> Iterator for Iter<T> {
    type Item = *mut ListHead<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head.is_null() {
            return None
        } else {
            let litem = self.curr as *mut ListHead<T>;
            if litem == self.head as *mut ListHead<T> && self.passed {
                None
            } else {
                self.passed = true;
                self.curr = unsafe { (*litem).next };
                Some(litem)
            }
        }
    }
}

// TODO: impl Rev<'a> for Iter<'a> { }
// TODO: impl Rev<'a> for IterMut<'a> { }
