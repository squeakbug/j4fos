use mm::zone::{next_aligned_by, Zone, PAGE_SHIFT, PAGE_SIZE};

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
    let mut heap = vec![0u8; (PAGE_SIZE << 10) + PAGE_SIZE];
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
fn test_heap_alloc_and_free_different_sizes_raising() {
    let mut heap = vec![0u8; (PAGE_SIZE << 10) + PAGE_SIZE];
    let mut zone = Zone::new();
    unsafe {
        let start_heap = (&mut heap[0]) as *mut u8 as usize;
        let start = next_aligned_by(start_heap, PAGE_SIZE);
        let end = start + (1 << PAGE_SHIFT << 10);
        zone.add_to_heap(start, end);
    }
    for order in 0..2 {
        let addr = zone.alloc_pages(order).unwrap();
        zone.free_pages(addr, order);
    }
}

#[test]
fn test_heap_alloc_and_free_different_sizes_lowering() {
    let mut heap = vec![0u8; (PAGE_SIZE << 10) + PAGE_SIZE];
    let mut zone = Zone::new();
    unsafe {
        let start_heap = (&mut heap[0]) as *mut u8 as usize;
        let start = next_aligned_by(start_heap, PAGE_SIZE);
        let end = start + (1 << PAGE_SHIFT << 10);
        zone.add_to_heap(start, end);
    }
    for order in (0..5).rev() {
        let addr = zone.alloc_pages(order).unwrap();
        zone.free_pages(addr, order);
    }
}

#[test]
fn test_heap_alloc_and_free_different_sizes_random() {
    let mut heap = vec![0u8; (PAGE_SIZE << 10) + PAGE_SIZE];
    let mut zone = Zone::new();
    unsafe {
        let start_heap = (&mut heap[0]) as *mut u8 as usize;
        let start = next_aligned_by(start_heap, PAGE_SIZE);
        let end = start + (PAGE_SIZE << 10);
        zone.add_to_heap(start, end);
    }

    let mut addrs = vec![];
    for order in (0..6).rev() {
        let addr = zone.alloc_pages(order).unwrap();
        addrs.push((addr, order));
    }

    for order in 0..6 {
        let addr = zone.alloc_pages(order).unwrap();
        addrs.push((addr, order));
    }

    for (addr, order) in addrs.into_iter() {
        zone.free_pages(addr, order);
    }
}
