#![allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use leet_code::Solution;

//// Simple case
// pub fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("sort_string 500", |b| {
//         b.iter(|| Solution::sort_string(black_box("a".repeat(500))))
//     });
// }
// criterion_group!(benches, criterion_benchmark);

//// Group from 2 functions and range of input
// fn criterion_benchmark_2(c: &mut Criterion) {
//     let mut group = c.benchmark_group("1365. How Many Numbers Are Smaller Than the Current Number");
//     for i in [100, 250, 1000].iter() {
//         let v = (0..=100).cycle().take(*i as usize).collect::<Vec<i32>>();

//         group.bench_with_input(BenchmarkId::new("Binary search", i), &v, |b, v| {
//             b.iter(|| Solution::smaller_numbers_than_current(v.clone()))
//         });

//         group.bench_with_input(BenchmarkId::new("HashMap", i), &v, |b, v| {
//             b.iter(|| Solution::smaller_numbers_than_current_hashmap(v.clone()))
//         });

//         group.bench_with_input(BenchmarkId::new("Running sum", i), &v, |b, v| {
//             b.iter(|| Solution::smaller_numbers_than_current_running_sum(v.clone()))
//         });
//     }
//     group.finish();
// }

// Group from 2 functions and range of input
fn criterion_benchmark_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("1356. Sort Integers by The Number of 1 Bits");
    for i in [500, 1000, 10_000].iter() {
        let v = (0..=*i).collect::<Vec<i32>>();

        group.bench_with_input(BenchmarkId::new("Sort with cmp and then", i), &v, |b, v| {
            b.iter(|| Solution::sort_by_bits(v.clone()))
        });

        group.bench_with_input(BenchmarkId::new("Sort with tuple", i), &v, |b, v| {
            b.iter(|| Solution::sort_by_bits(v.clone()))
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark_2);

criterion_main!(benches);
