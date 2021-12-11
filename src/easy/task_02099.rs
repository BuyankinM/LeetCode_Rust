// 2099. Find Subsequence of Length K With the Largest Sum
// https://leetcode.com/problems/find-subsequence-of-length-k-with-the-largest-sum/

use crate::Solution;

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == nums.len() as i32 || nums.len() == 1 {
            return nums;
        }

        let mut num_idx = nums.iter().enumerate().collect::<Vec<_>>();
        num_idx.sort_unstable_by_key(|(_, x)| **x);

        let mut idx = vec![false; nums.len()];
        num_idx
            .into_iter()
            .rev()
            .take(k as usize)
            .for_each(|(i, _)| idx[i] = true);

        nums.into_iter()
            .enumerate()
            .filter_map(|(i, x)| if idx[i] { Some(x) } else { None })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![3, 3], Solution::max_subsequence(vec![2, 1, 3, 3], 2));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![-1, 3, 4],
            Solution::max_subsequence(vec![-1, -2, 3, 4], 3)
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![4, 3, 0],
            Solution::max_subsequence(vec![-1, -2, 4, 3, 0], 3)
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(vec![4, 3], Solution::max_subsequence(vec![3, 4, 3, 3], 2));
    }
}
