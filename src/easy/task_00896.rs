// 896. Monotonic Array
// https://leetcode.com/problems/monotonic-array/

use crate::Solution;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        use std::cmp::Ordering;
        let mut dir = 0;
        for (a, b) in nums.iter().zip(nums[1..].iter()) {
            dir |= match a.cmp(b) {
                Ordering::Equal => 0,
                Ordering::Less => 1,
                Ordering::Greater => 2,
            };
            if dir == 3 {
                return false;
            }
        }
        true
    }

    pub fn is_monotonic_fp(nums: Vec<i32>) -> bool {
        use std::cmp::Ordering;
        match nums.len() {
            1 => true,
            _ => nums
                .iter()
                .zip(nums[1..].iter())
                .try_fold(0, |mut dir, (a, b)| {
                    dir |= match a.cmp(b) {
                        Ordering::Equal => 0b000,
                        Ordering::Less => 0b001,
                        Ordering::Greater => 0b010,
                    };
                    if dir == 0b011 {
                        None
                    } else {
                        Some(dir)
                    }
                })
                .is_some(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(Solution::is_monotonic(vec![1, 2, 2, 3]));
    }

    #[test]
    fn test_2() {
        assert!(Solution::is_monotonic(vec![6, 5, 4, 4]));
    }

    #[test]
    fn test_3() {
        assert!(!Solution::is_monotonic(vec![1, 3, 2]));
    }

    #[test]
    fn test_4() {
        assert!(Solution::is_monotonic(vec![1, 2, 4, 5]));
    }

    #[test]
    fn test_5() {
        assert!(Solution::is_monotonic(vec![1, 1, 1]));
    }

    #[test]
    fn test_6() {
        assert!(Solution::is_monotonic(vec![2]));
    }
}
