#![feature(allocator_api)]
#![feature(ptr_as_uninit)]
#![feature(slice_ptr_get)]
#![feature(ptr_alignment_type)]

use std::{
    alloc::{Allocator, Global, Layout},
    fs::File,
    io::Read,
};

pub struct MyAlloc;

unsafe impl Allocator for MyAlloc {
    fn allocate(&self, layout: Layout) -> Result<std::ptr::NonNull<[u8]>, std::alloc::AllocError> {
        Global.allocate(unsafe {
            Layout::from_size_align_unchecked(layout.size(), layout.align().max(32))
        })
    }

    unsafe fn deallocate(&self, ptr: std::ptr::NonNull<u8>, layout: Layout) {
        Global.deallocate(ptr, unsafe {
            Layout::from_size_align_unchecked(layout.size(), layout.align().max(32))
        })
    }
}

pub fn std_read_32_byte_aligned(path: &str) -> Vec<u8, MyAlloc> {
    let mut f = File::open(path).unwrap();
    let len = f.metadata().unwrap().len() as usize;

    let mut buf = Vec::with_capacity_in(len, MyAlloc {});

    let p = std::ptr::slice_from_raw_parts_mut(buf.as_mut_ptr(), len);

    let did_read = f.read(unsafe { &mut *p }).unwrap();
    assert_eq!(did_read, len); // could fail

    let mut probe_buf = [0u8; 32];
    let did_read = f.read(&mut probe_buf).unwrap();
    assert_eq!(did_read, 0);

    drop(f);

    unsafe { buf.set_len(len) };
    buf
}

pub fn std_read(path: &str) -> Vec<u8> {
    let mut f = File::open(path).unwrap();
    let len = f.metadata().unwrap().len() as usize;

    let mut buf = Vec::with_capacity(len);

    let p = std::ptr::slice_from_raw_parts_mut(buf.as_mut_ptr(), len);

    let did_read = f.read(unsafe { &mut *p }).unwrap();
    assert_eq!(did_read, len); // could fail

    let mut probe_buf = [0u8; 32];
    let did_read = f.read(&mut probe_buf).unwrap();
    assert_eq!(did_read, 0);

    drop(f);

    unsafe { buf.set_len(len) };
    buf
}
