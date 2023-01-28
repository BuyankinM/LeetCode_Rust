// #![allow(unused_imports)]
// #![allow(dead_code)]
// use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
// use leet_code::Solution;

// pub fn criterion_benchmark_task_1370(c: &mut Criterion) {
//     c.bench_function("sort_string 500", |b| {
//         b.iter(|| Solution::sort_string(black_box("a".repeat(500))))
//     });
// }

// fn criterion_benchmark_task_2016(c: &mut Criterion) {
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

// fn criterion_benchmark_task_2022(c: &mut Criterion) {
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

// fn criterion_benchmark_task_0367(c: &mut Criterion) {
//     let mut group = c.benchmark_group("367. Valid Perfect Square");
//     for i in [100, 250_000, 808_201, 2_147_483_647].iter() {
//         group.bench_with_input(BenchmarkId::new("Binary search", i), i, |b, i| {
//             b.iter(|| Solution::is_perfect_square(*i))
//         });

//         group.bench_with_input(BenchmarkId::new("Functional (SQRT(N))", i), i, |b, i| {
//             b.iter(|| Solution::is_perfect_square_func(*i))
//         });

//         group.bench_with_input(BenchmarkId::new("Sum of 1_3_5_7", i), i, |b, i| {
//             b.iter(|| Solution::is_perfect_square_1357(*i))
//         });

//         group.bench_with_input(BenchmarkId::new("Newton method", i), i, |b, i| {
//             b.iter(|| Solution::is_perfect_square_newton(*i))
//         });
//     }
//     group.finish();
// }

// pub fn criterion_benchmark_task_1091(c: &mut Criterion) {
//     let mut group = c.benchmark_group("1091. Shortest Path in Binary Matrix");

//     for i in [50, 100, 200].iter() {
//         let n = *i as usize;
//         let v = vec![vec![0; n]; n];
//         group.bench_with_input(BenchmarkId::new("Solution with heap", i), &v, |b, v| {
//             b.iter(|| Solution::shortest_path_binary_matrix(v.clone()))
//         });
//     }

//     group.finish();
// }

// fn criterion_benchmark_task_2542(c: &mut Criterion) {
//     use rand::{distributions::Uniform, Rng};

//     let range = Uniform::from(0..100_000);

//     let nums1 = rand::thread_rng()
//         .sample_iter(&range)
//         .take(100_000)
//         .collect::<Vec<i32>>();

//     let nums2 = rand::thread_rng()
//         .sample_iter(&range)
//         .take(100_000)
//         .collect::<Vec<i32>>();

//     let mut group = c.benchmark_group("2542. Maximum Subsequence Score");
//     for &i in [1000, 10_000, 90_000].iter() {
//         let n1 = nums1.iter().take(i).cloned().collect::<Vec<i32>>();
//         let n2 = nums2.iter().take(i).cloned().collect::<Vec<i32>>();
//         let k = 1000;

//         group.bench_with_input(BenchmarkId::new("Before optimization", i), &k, |b, &k| {
//             b.iter(|| Solution::max_score_2542_opt(n1.clone(), n2.clone(), k))
//         });

//         group.bench_with_input(BenchmarkId::new("After optimization", i), &k, |b, &k| {
//             b.iter(|| Solution::max_score_2542_opt_2(n1.clone(), n2.clone(), k))
//         });
//     }
//     group.finish();
// }

// criterion_group!(benches, criterion_benchmark_task_2542);

// criterion_main!(benches);
