// 2016. Maximum Difference Between Increasing Elements
// https://leetcode.com/problems/maximum-difference-between-increasing-elements/

use crate::Solution;

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut diff = -1;
        let (mut max, mut min) = (nums[0], nums[0]);
        for &val in nums[1..].iter() {
            if val < min {
                min = val;
                max = val;
            } else if val > max {
                max = val;
                diff = diff.max(max - min);
            }
        }
        diff
    }

    // https://leetcode.com/problems/maximum-difference-between-increasing-elements/discuss/1492046/Rust-solution/1101698
    pub fn maximum_difference_func(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((-1, i32::MAX), |(res, mn), &n| {
                let diff = if n > mn { n - mn } else { -2 };
                (std::cmp::max(res, diff), std::cmp::min(mn, n))
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::maximum_difference(vec![7, 1, 5, 4]));
    }

    #[test]
    fn test_2() {
        assert_eq!(-1, Solution::maximum_difference(vec![9, 4, 3, 2]));
    }

    #[test]
    fn test_3() {
        assert_eq!(9, Solution::maximum_difference(vec![1, 5, 2, 10]));
    }
}
