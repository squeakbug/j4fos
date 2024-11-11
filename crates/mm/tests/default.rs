use std::{
    alloc::Layout,
    mem::size_of,
};

use mm::Zone;

const MACHINE_ALIGN: usize = core::mem::size_of::<usize>();
const HEAP_SIZE: usize = 4 * 1024 * 1024;
const HEAP_BLOCK: usize = HEAP_SIZE / MACHINE_ALIGN;
static mut HEAP: [usize; HEAP_BLOCK] = [0; HEAP_BLOCK];

#[test]
fn test_empty_heap() {
    let mut heap = Zone::new();
    assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_err());
}

#[test]
fn test_heap_add() {
    let mut heap = Zone::new();
    assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_err());

    let space: [usize; 100] = [0; 100];
    unsafe {
        heap.add_to_heap(space.as_ptr() as usize, space.as_ptr().add(100) as usize);
    }
    let addr = heap.alloc(Layout::from_size_align(1, 1).unwrap());
    assert!(addr.is_ok());
}

#[test]
fn test_heap_add_large() {
    let mut heap = Zone::new();
    assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_err());

    let space: [u8; 512] = [0; 512];
    unsafe {
        heap.add_to_heap(space.as_ptr() as usize, space.as_ptr().add(512) as usize);
    }
    let addr = heap.alloc(Layout::from_size_align(1, 1).unwrap());
    assert!(addr.is_ok());
}

#[test]
fn test_heap_oom() {
    let mut heap = Zone::new();
    let space: [usize; 100] = [0; 100];
    unsafe {
        heap.add_to_heap(space.as_ptr() as usize, space.as_ptr().add(100) as usize);
    }

    assert!(heap
        .alloc(Layout::from_size_align(100 * size_of::<usize>(), 1).unwrap())
        .is_err());
    assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_ok());
}

#[test]
fn test_heap_alloc_and_free() {
    let mut heap = Zone::new();
    assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_err());

    let space: [usize; 100] = [0; 100];
    unsafe {
        heap.add_to_heap(space.as_ptr() as usize, space.as_ptr().add(100) as usize);
    }
    for _ in 0..100 {
        let addr = heap.alloc(Layout::from_size_align(1, 1).unwrap()).unwrap();
        heap.dealloc(addr, Layout::from_size_align(1, 1).unwrap());
    }
}

#[test]
fn test_heap_alloc_and_free_different_sizes() {
    let mut heap = Zone::new();
    assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_err());

    unsafe {
        heap.add_to_heap(HEAP.as_ptr() as usize, HEAP.as_ptr().add(1 << 16) as usize);
    }
    for block_size in 1..12 {
        let addr = heap.alloc(Layout::from_size_align(1 << block_size, 1).unwrap()).unwrap();
        heap.dealloc(addr, Layout::from_size_align(1 << block_size, 1).unwrap());
    }
}

#[test]
fn test_heap_alloc_and_free_different_sizes_lowering() {
    let mut heap = Zone::new();
    assert!(heap.alloc(Layout::from_size_align(1, 1).unwrap()).is_err());

    unsafe {
        heap.add_to_heap(HEAP.as_ptr() as usize, HEAP.as_ptr().add(1 << 16) as usize);
    }
    for block_size in (12..1).rev() {
        let addr = heap.alloc(Layout::from_size_align(1 << block_size, 1).unwrap()).unwrap();
        heap.dealloc(addr, Layout::from_size_align(1 << block_size, 1).unwrap());
    }
}

// TODO: add test with scatter/gather loading/storing with different sizes
