// 923. 3Sum With Multiplicity
// https://leetcode.com/problems/3sum-with-multiplicity/

use crate::Solution;
use std::time::Instant;

const M: i32 = 1_000_000_007;

impl Solution {
    pub fn three_sum_multi_bruteforce(arr: Vec<i32>, target: i32) -> i32 {
        let l = arr.len();
        let mut res: i32 = 0;

        for i in 0..l - 2 {
            let ai = arr[i];
            for j in (i + 1)..l - 1 {
                let aj = arr[j];
                for &ak in arr[j + 1..].iter() {
                    match ai + aj + ak {
                        s if s == target => {
                            res += 1;
                            res %= M;
                        }
                        s if (s > target) => break,
                        _ => (),
                    }
                }
            }
        }

        res % M
    }

    pub fn three_sum_multi_3pointers(arr: Vec<i32>, target: i32) -> i32 {
        let len = arr.len();
        let mut res: i32 = 0;

        for i in 0..len {
            let t = target - arr[i];
            let (mut j, mut k) = (i + 1, len - 1);

            while j < k {
                if arr[j] + arr[k] < t {
                    j += 1;
                } else if arr[j] + arr[k] > t {
                    k -= 1;
                } else if arr[j] != arr[k] {
                    let (mut left, mut right) = (1, 1);

                    while j + 1 < k && arr[j] == arr[j + 1] {
                        left += 1;
                        j += 1;
                    }
                    while k - 1 > j && arr[k] == arr[k - 1] {
                        right += 1;
                        k -= 1;
                    }

                    res += left * right;
                    res %= M;

                    j += 1;
                    k -= 1;
                } else {
                    res += ((k - j + 1) * (k - j) / 2) as i32;
                    res %= M;
                    break;
                }
            }
        }

        res % M
    }

    pub fn three_sum_multi_cases(arr: Vec<i32>, target: i32) -> i32 {
        let modd = 1_000_000_007;
        let mut counts = vec![0_i64; 101];
        arr.iter().for_each(|&n| counts[n as usize] += 1);
        let mut answer = 0;
        for i in 0..=100 {
            for j in i..=100 {
                let k = target - i - j;
                if (j..=100).contains(&k) {
                    let (ci, cj, ck) = (counts[i as usize], counts[j as usize], counts[k as usize]);
                    answer += match (i == j, j == k) {
                        (false, false) => ci * cj * ck,
                        (false, true) => ci * cj * (cj - 1) / 2,
                        (true, false) => ci * (ci - 1) / 2 * ck,
                        (true, true) => ci * (ci - 1) * (ci - 2) / 6,
                    };
                    answer %= modd;
                }
            }
        }
        answer as i32
    }
}
