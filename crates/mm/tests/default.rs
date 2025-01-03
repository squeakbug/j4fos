use mm::zone::{Zone, PAGE_SIZE, next_aligned_by};

#[test]
fn test_empty_heap() {
    let mut heap = Zone::new();
    assert!(heap.alloc_pages(1).is_err());
}

#[test]
fn test_heap_add() {
    let mut heap = vec![0u8; PAGE_SIZE << 5];
    let mut zone = Zone::new();
    unsafe {
        let start_heap = (&mut heap[0]) as *mut u8;
        zone.add_to_heap(
            next_aligned_by(start_heap as usize, PAGE_SIZE), 
            next_aligned_by(start_heap.add(PAGE_SIZE << 1) as usize, PAGE_SIZE)
        );
    }
    let addr = zone.alloc_pages(1);
    assert!(addr.is_ok());
}

#[test]
fn test_heap_add_exact() {
    let mut heap = vec![0u8; PAGE_SIZE << 5];
    let mut zone = Zone::new();
    unsafe {
        let start_heap = (&mut heap[0]) as *mut u8;
        zone.add_to_heap(
            next_aligned_by(start_heap as usize, PAGE_SIZE), 
            next_aligned_by(start_heap.add(PAGE_SIZE * 3) as usize, PAGE_SIZE)
        );
    }
    let addr = zone.alloc_pages(1);
    assert!(addr.is_ok());
}

#[test]
fn test_heap_oom() {
    let mut heap = vec![0u8; PAGE_SIZE];
    let mut zone = Zone::new();
    unsafe {
        let start_heap = (&mut heap[0]) as *mut u8;
        zone.add_to_heap(
            next_aligned_by(start_heap as usize, PAGE_SIZE), 
            next_aligned_by(start_heap.add(PAGE_SIZE << 1) as usize, PAGE_SIZE)
        );
    }
    assert!(zone.alloc_pages(2).is_err());
    assert!(zone.alloc_pages(1).is_ok());
}

#[test]
fn test_heap_alloc_and_free() {
    let mut heap = vec![0u8; PAGE_SIZE << 5];
    let mut zone = Zone::new();
    unsafe {
        let start_heap = (&mut heap[0]) as *mut u8;
        zone.add_to_heap(
            next_aligned_by(start_heap as usize, PAGE_SIZE), 
            next_aligned_by(start_heap.add(PAGE_SIZE << 1) as usize, PAGE_SIZE)
        );
    }
    for _ in 0..100 {
        let addr = zone.alloc_pages(1).unwrap();
        zone.free_pages(addr, 1);
    }
}

#[test]
fn test_heap_alloc_and_free_different_sizes() {
    let mut heap = vec![0u8; PAGE_SIZE << 5];
    let mut zone = Zone::new();
    unsafe {
        let start_heap = (&mut heap[0]) as *mut u8;
        zone.add_to_heap(
            next_aligned_by(start_heap as usize, PAGE_SIZE), 
            next_aligned_by(start_heap.add(PAGE_SIZE << 5) as usize, PAGE_SIZE)
        );
    }
    for order in 0..5 {
        let addr = zone.alloc_pages(order).unwrap();
        zone.free_pages(addr, order);
    }
}

#[test]
fn test_heap_alloc_and_free_different_sizes_lowering() {
    let mut heap = vec![0u8; PAGE_SIZE << 5];
    let mut zone = Zone::new();
    unsafe {
        let start_heap = (&mut heap[0]) as *mut u8;
        zone.add_to_heap(
            next_aligned_by(start_heap as usize, PAGE_SIZE), 
            next_aligned_by(start_heap.add(PAGE_SIZE << 5) as usize, PAGE_SIZE)
        );
    }
    for order in (0..6).rev() {
        let addr = zone.alloc_pages(order).unwrap();
        zone.free_pages(addr, order);
    }
}
