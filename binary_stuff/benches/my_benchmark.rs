use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

#[path ="../src/lib.rs"]
mod lib;


fn from_elem(c: &mut Criterion) {
    let data: Vec<u8> = vec![0, 2, 100, 35, 11, 12, 8, 1, 0, 0, 0, 99, 97, 113, 111, 2, 2, 8];

    c.bench_with_input(BenchmarkId::new("count_zeros", "data_vec"), &data, |b, s| {
        b.iter(|| lib::count_zeros(&s));
    });
}

criterion_group!(benches, from_elem);
criterion_main!(benches);
