// 643. Maximum Average Subarray I
// https://leetcode.com/problems/maximum-average-subarray-i/

use crate::Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let ku = k as usize;
        let mut s = nums[..ku].iter().sum::<i32>();
        let mut max_s = s;

        for (&next, &prev) in nums[ku..].iter().zip(nums.iter()) {
            s += next - prev;
            if s > max_s {
                max_s = s;
            }
        }
        max_s as f64 / k as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            12.75000,
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(5.0000, Solution::find_max_average(vec![5], 1));
    }

    #[test]
    fn test_3() {
        assert_eq!(4.0000, Solution::find_max_average(vec![0, 4, 0, 3, 2], 1));
    }
}
