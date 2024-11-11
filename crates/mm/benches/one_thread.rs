use std::thread;

use criterion::{criterion_group, criterion_main, Criterion};

use mm::LockedZone;

mod shared;

const ORDER: usize = 32;
const MACHINE_ALIGN: usize = core::mem::size_of::<usize>();
const HEAP_SIZE: usize = 1024 * 1024 * 1024;
const HEAP_BLOCK: usize = HEAP_SIZE / MACHINE_ALIGN;
static mut HEAP: [usize; HEAP_BLOCK] = [0; HEAP_BLOCK];

const THREAD_SIZE: usize = 2;

#[global_allocator]
static BUDDY_ALLOCATOR: LockedZone = LockedZone::new();

#[ctor::ctor]
fn init_heap() {
    let heap_start = unsafe { HEAP.as_ptr() as usize };
    unsafe {
        BUDDY_ALLOCATOR
            .lock()
            .init(heap_start, HEAP_BLOCK * MACHINE_ALIGN);
    }
}

fn multi_thread_random_size() {
    let _ = (0..THREAD_SIZE)
        .into_iter()
        .map(|_indx| {
            thread::spawn(move || shared::random_workload())
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|handle| handle.join().unwrap());
}

fn multi_thread_heavy_size() {
    let _ = (0..THREAD_SIZE)
        .into_iter()
        .map(|_indx| {
            thread::spawn(move || shared::heavy_workload())
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|handle| handle.join().unwrap());
}

fn multi_thread_light_size() {
    let _ = (0..THREAD_SIZE)
        .into_iter()
        .map(|_indx| {
            thread::spawn(move || shared::light_workload())
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|handle| handle.join().unwrap());
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("multi thread random size", |b| {
        b.iter(|| multi_thread_random_size())
    });

    c.bench_function("multi thread heavy size", |b| {
        b.iter(|| multi_thread_heavy_size())
    });

    c.bench_function("multi thread small size", |b| {
        b.iter(|| multi_thread_light_size())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
