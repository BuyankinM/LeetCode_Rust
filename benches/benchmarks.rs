#![allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use leet_code::Solution;

// Simple case
// pub fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("count_largest_group 9999", |b| {
//         b.iter(|| Solution::count_largest_group(black_box(9999)))
//     });
// }

// Group from 2 functions and range of input
fn criterion_benchmark_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("1399. Count Largest Group");
    for i in [1000, 9999].iter() {
        group.bench_with_input(BenchmarkId::new("Function Sum", i), i, |b, i| {
            b.iter(|| Solution::count_largest_group(*i))
        });
        group.bench_with_input(BenchmarkId::new("DP", i), i, |b, i| {
            b.iter(|| Solution::count_largest_group_dp(*i))
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark_2);
criterion_main!(benches);
