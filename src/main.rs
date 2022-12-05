#![feature(allocator_api)]
#![feature(ptr_as_uninit)]

use std::{alloc::{Layout, Allocator}, time::Instant};

use nix::{fcntl::OFlag, sys::stat::Mode};
use rand::{Rng, distributions::Uniform};

fn main() {
    let mut rng = rand::thread_rng();
    let mut s = Vec::<u8>::new();
    for _ in 0..400_000_000 {
        s.push(rng.sample(Uniform::new(0, 26)) + 'a' as u8);
    }
    let path = "testfile";
    std::fs::write(path, s).unwrap();
}