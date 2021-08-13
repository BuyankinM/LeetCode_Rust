// 1304. Find N Unique Integers Sum up to Zero
// https://leetcode.com/problems/find-n-unique-integers-sum-up-to-zero/

use crate::Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        (1..n).chain(std::iter::once(-n * (n - 1) / 2)).collect()
    }

    pub fn sum_zero_short(n: i32) -> Vec<i32> {
        (1 - n..n).step_by(2).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![1, 2, 3, 4, -10], Solution::sum_zero(5));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![1, 2, -3], Solution::sum_zero(3));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![-2, 0, 2], Solution::sum_zero_short(3));
    }
}
