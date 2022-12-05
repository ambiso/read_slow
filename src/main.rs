#![feature(allocator_api)]
#![feature(ptr_as_uninit)]

use rand::{distributions::Uniform, Rng};

fn main() {
    let mut rng = rand::thread_rng();
    let mut s = Vec::<u8>::new();
    for _ in 0..400_000_000 {
        s.push(rng.sample(Uniform::new(0, 26)) + 'a' as u8);
    }
    let path = "testfile";
    std::fs::write(path, s).unwrap();
}
