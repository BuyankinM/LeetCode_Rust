// 1486. XOR Operation in an Array
// https://leetcode.com/problems/xor-operation-in-an-array/

use crate::Solution;

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        (1..n)
            .fold((start, start), |(acc, val), _| (acc ^ (val + 2), val + 2))
            .0
    }

    pub fn xor_operation_2(n: i32, start: i32) -> i32 {
        (0..n)
            .fold((0, start), |(acc, val), _| (acc ^ val, val + 2))
            .0
    }

    pub fn xor_operation_best(n: i32, start: i32) -> i32 {
        (0..n).fold(0, |acc, i| acc ^ (start + 2 * i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(8, Solution::xor_operation(5, 0));
    }

    #[test]
    fn test_2() {
        assert_eq!(8, Solution::xor_operation_2(4, 3));
    }

    #[test]
    fn test_3() {
        assert_eq!(7, Solution::xor_operation_best(1, 7));
    }
}
