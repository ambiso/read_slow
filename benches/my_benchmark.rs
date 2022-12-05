#![feature(allocator_api)]
#![feature(ptr_as_uninit)]

use nix::{fcntl::OFlag, sys::stat::Mode};
use std::{alloc::{Allocator, Layout}, io::Read};

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let path = "testfile";
    c.bench_function("std::fs::read", |b| b.iter(|| std::fs::read(path)));
    c.bench_function("terrible hack", |b| {
        b.iter(|| {
            let f = std::fs::File::open(path).unwrap();
            let n = f.metadata().unwrap().len() as usize;
            let a = std::alloc::System;
            let layout = Layout::from_size_align(n, 32).unwrap();
            let v = a.allocate(layout).unwrap();
            let fd = nix::fcntl::open(path, OFlag::O_RDONLY, Mode::empty()).unwrap();
            let v1 = unsafe { std::mem::transmute::<_, &mut [u8]>(v.as_uninit_slice_mut()) };
            let x = nix::unistd::read(fd, v1).unwrap();
            assert_eq!(x, n); // I know this can fail
            unsafe { Vec::from_raw_parts(v.cast::<u8>().as_ptr(), n, n) }
        })
    });
    c.bench_function("safe hack", |b| {
        b.iter(|| {
            let mut f = std::fs::File::open(path).unwrap();
            let n = f.metadata().unwrap().len() as usize;
            let mut v3 = vec![0u8; n];
            let x = f.read(&mut v3).unwrap();
            assert_eq!(x, n);
            v3
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
