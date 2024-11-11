use rand::Rng;

const SMALL_SIZE: usize = 8;
const LARGE_SIZE: usize = 1024 * 64;
const REPEAT_CNT: usize = 100;

pub fn random_workload() {
    let mut rng = rand::thread_rng();

    for _ in 0..REPEAT_CNT {
        let size = rng.gen_range(SMALL_SIZE..=LARGE_SIZE);
        let mut vec: Vec<i32> = Vec::with_capacity(size);
        for _ in 0..size {
            vec.push(rng.gen_range(1..=1_000_000));
        }
        vec.sort();
    }
}

pub fn heavy_workload() {
    let mut rng = rand::thread_rng();

    for _ in 0..REPEAT_CNT {
        let size = LARGE_SIZE;
        let mut vec: Vec<i32> = Vec::with_capacity(size);
        for _ in 0..size {
            vec.push(rng.gen_range(1..=1_000_000));
        }
        vec.sort();
    }
}

pub fn light_workload() {
    let mut rng = rand::thread_rng();

    for _ in 0..REPEAT_CNT {
        let size = SMALL_SIZE;
        let mut vec: Vec<i32> = Vec::with_capacity(size);
        for _ in 0..size {
            vec.push(rng.gen_range(1..=1_000_000));
        }
        vec.sort();
    }
}
