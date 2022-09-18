// 2413. Smallest Even Multiple
// https://leetcode.com/problems/smallest-even-multiple/

use crate::Solution;

impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        match n % 2 {
            1 => n * 2,
            _ => n,
        }
    }

    pub fn smallest_even_multiple_oneline(n: i32) -> i32 {
        n << (n & 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::smallest_even_multiple(5), 10);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::smallest_even_multiple(6), 6);
    }
}
