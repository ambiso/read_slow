use criterion::{criterion_group, criterion_main, Criterion};
use read_slow::{std_read, std_read_32_byte_aligned};

fn criterion_benchmark(c: &mut Criterion) {
    let path = "testfile";
    c.bench_function("std_read 1 byte aligned", |b| b.iter(|| std_read(path)));
    c.bench_function("std_read 32 byte aligned", |b| b.iter(|| std_read_32_byte_aligned(path)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
