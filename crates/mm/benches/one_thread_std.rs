use std::thread;

use criterion::{criterion_group, criterion_main, Criterion};

mod shared;

const THREAD_SIZE: usize = 2;

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
