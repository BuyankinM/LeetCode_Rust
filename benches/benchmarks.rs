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
// fn criterion_benchmark_2(c: &mut Criterion) {
//     let mut group = c.benchmark_group("1356. Sort Integers by The Number of 1 Bits");
//     for i in [500, 1000, 10_000].iter() {
//         let v = (0..=*i).collect::<Vec<i32>>();

//         group.bench_with_input(BenchmarkId::new("Sort with cmp and then", i), &v, |b, v| {
//             b.iter(|| Solution::sort_by_bits(v.clone()))
//         });

//         group.bench_with_input(BenchmarkId::new("Sort with tuple", i), &v, |b, v| {
//             b.iter(|| Solution::sort_by_bits(v.clone()))
//         });
//     }
//     group.finish();
// }

// fn criterion_benchmark_2(c: &mut Criterion) {
//     use rand::{distributions::Uniform, Rng};

//     let range = Uniform::from(0..10_000);
//     let vals = rand::thread_rng()
//         .sample_iter(&range)
//         .take(100_000)
//         .collect::<Vec<i32>>();

//     let mut group = c.benchmark_group("2016. Maximum Difference Between Increasing Elements");
//     for &i in [500, 1000, 10_000].iter() {
//         let v = vals.iter().take(i as usize).cloned().collect::<Vec<i32>>();

//         group.bench_with_input(BenchmarkId::new("Simple cycle", i), &v, |b, v| {
//             b.iter(|| Solution::maximum_difference(v.clone()))
//         });

//         group.bench_with_input(BenchmarkId::new("Functional", i), &v, |b, v| {
//             b.iter(|| Solution::maximum_difference_func(v.clone()))
//         });
//     }
//     group.finish();
// }

// Group from 2 functions and range of input
// fn criterion_benchmark_2(c: &mut Criterion) {
//     let mut group = c.benchmark_group("2022. Convert 1D Array Into 2D Array");
//     for &i in [100, 500, 1000].iter() {
//         let m = i as i32;
//         let m2 = m * m;
//         let v = (0..m2).collect::<Vec<i32>>();

//         group.bench_with_input(BenchmarkId::new("Chunks", &m2), &v, |b, v| {
//             b.iter(|| Solution::construct2_d_array(v.clone(), i, i))
//         });

//         group.bench_with_input(BenchmarkId::new("Iter_Take", &m2), &v, |b, v| {
//             b.iter(|| Solution::construct2_d_array(v.clone(), i, i))
//         });
//     }
//     group.finish();
// }

fn criterion_benchmark_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("2027. Minimum Moves to Convert String");
    for &i in [1000, 5000, 10000].iter() {
        let l = 4 * i;
        let s = "XXOX".repeat(i as usize);

        group.bench_with_input(BenchmarkId::new("Into_Bytes", &l), &s, |b, s| {
            b.iter(|| Solution::minimum_moves(s.clone()))
        });

        group.bench_with_input(BenchmarkId::new("Chars", &l), &s, |b, s| {
            b.iter(|| Solution::minimum_moves_chars(s.clone()))
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark_2);

criterion_main!(benches);
