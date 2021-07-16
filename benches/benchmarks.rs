use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leet_code::Solution;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("count_largest_group 9999", |b| {
        b.iter(|| Solution::count_largest_group(black_box(9999)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
