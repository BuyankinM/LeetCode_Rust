// 2553. Separate the Digits in an Array
// https://leetcode.com/problems/separate-the-digits-in-an-array/

use crate::Solution;

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        let mut dig_vec = vec![];
        for mut n in nums {
            while n > 0 {
                dig_vec.push(n % 10);
                n /= 10;
            }
            result.extend(dig_vec.iter().cloned().rev());
            dig_vec.clear();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![1, 3, 2, 5, 8, 3, 7, 7],
            Solution::separate_digits(vec![13, 25, 83, 77])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![7, 1, 3, 9],
            Solution::separate_digits(vec![7, 1, 3, 9])
        );
    }
}
