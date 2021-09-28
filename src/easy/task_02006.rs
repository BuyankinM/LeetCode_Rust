// 2006. Count Number of Pairs With Absolute Difference K
// https://leetcode.com/problems/count-number-of-pairs-with-absolute-difference-k/

use crate::Solution;

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter()
            .enumerate()
            .map(|(ind, &x)| {
                nums[ind + 1..]
                    .iter()
                    .filter(|&&y| (x - y).abs() == k)
                    .count() as i32
            })
            .sum::<_>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::count_k_difference(vec![1, 2, 2, 1], 1));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::count_k_difference(vec![1,3], 3));
    }

    #[test]
    fn test_3() {
        assert_eq!(3, Solution::count_k_difference(vec![3,2,1,5,4], 2));
    }
}
