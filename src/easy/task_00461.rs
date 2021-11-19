// 461. Hamming Distance
// https://leetcode.com/problems/hamming-distance/

use crate::Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::hamming_distance(1, 4));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::hamming_distance(3, 1));
    }

    #[test]
    fn test_3() {
        assert_eq!(8, Solution::hamming_distance(333, 55555));
    }
}
