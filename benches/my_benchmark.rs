#![feature(allocator_api)]
#![feature(ptr_as_uninit)]
#![feature(slice_ptr_get)]

use nix::{fcntl::OFlag, sys::stat::Mode};
use std::{
    alloc::{Allocator, Layout},
    io::Read,
};

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

            // Mimic probe read done by read_to_end
            let mut probe_buf = [0u8; 32];
            let x2 = nix::unistd::read(fd, &mut probe_buf).unwrap();
            assert_eq!(x2, 0);

            // Clean up
            nix::unistd::close(fd).unwrap();
            // unsafe { a.deallocate(v.as_non_null_ptr(), layout);}
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
