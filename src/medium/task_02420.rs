// 2420. Find All Good Indices
// https://leetcode.com/problems/find-all-good-indices/

use crate::Solution;

impl Solution {
    pub fn good_indices(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let l = nums.len();
        let ku = k as usize;

        let mut dp_front = vec![1; l];
        let mut dp_back = vec![1; l];

        (1..l)
            .filter(|&i| nums[i - 1] >= nums[i])
            .for_each(|i| dp_front[i] = dp_front[i - 1] + 1);
        (0..l - 1)
            .rev()
            .filter(|&i| nums[i + 1] >= nums[i])
            .for_each(|i| dp_back[i] = dp_back[i + 1] + 1);

        (ku..l - ku)
            .filter_map(|i| (dp_front[i - 1] >= k && dp_back[i + 1] >= k).then_some(i as i32))
            .collect()
    }

    // https://leetcode.com/problems/find-all-good-indices/discuss/2621754/Rust-or-Prefix-Arrays-or-With-Comments
    pub fn good_indices_scan(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let (n, k) = (nums.len(), k as usize);
        let dp_desc: Vec<_> = nums
            .iter()
            .scan((i32::MAX, 0), |(prev, dp), num| {
                *dp = if *prev >= *num { *dp + 1 } else { 1 };
                *prev = *num;
                Some(*dp)
            })
            .collect();
        let dp_asc: Vec<_> = nums
            .iter()
            .rev()
            .scan((i32::MAX, 0), |(prev, dp), num| {
                *dp = if *prev >= *num { *dp + 1 } else { 1 };
                *prev = *num;
                Some(*dp)
            })
            .collect::<Vec<_>>();
        (k..n - k)
            .filter_map(|i| (dp_desc[i - 1] >= k && dp_asc[n - i - 2] >= k).then_some(i as i32))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::good_indices(vec![2, 1, 1, 1, 3, 4, 1], 2),
            vec![2, 3]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::good_indices(vec![2, 1, 1, 2], 2), vec![]);
    }
}
