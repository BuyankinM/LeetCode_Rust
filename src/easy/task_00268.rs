// 268. Missing Number
// https://leetcode.com/problems/missing-number/

use crate::Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        (nums.len() * (nums.len() + 1) / 2) as i32 - nums.iter().sum::<i32>()
    }

    // https://leetcode.com/problems/missing-number/discuss/729784/Rust-Solutions
    pub fn missing_number_xor(nums: Vec<i32>) -> i32 {
        let mut miss = 0;
        for (i, &elem) in nums.iter().enumerate() {
            miss ^= elem ^ (i + 1) as i32;
        }
        miss
    }

    // https://leetcode.com/problems/missing-number/discuss/1091802/Rust%3A-0ms-O(n)-runtime-O(1)-space-XOR-solution
    pub fn missing_number_xor_func(nums: Vec<i32>) -> i32 {
        (1..=nums.len() as _)
            .zip(nums.into_iter())
            .fold(0, |xor, (x, y)| xor ^ x ^ y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::missing_number(vec![0, 1]), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
