// 2057. Smallest Index With Equal Value
// https://leetcode.com/problems/smallest-index-with-equal-value/

use crate::Solution;

impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        nums.iter()
            .zip(0..)
            .position(|(x, i)| i % 10 == *x)
            .map_or(-1, |i| i as i32)
    }

    pub fn smallest_equal_cycle(nums: Vec<i32>) -> i32 {
        nums.iter()
            .zip((0..10).cycle())
            .position(|(x, i)| i == *x)
            .map_or(-1, |i| i as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, Solution::smallest_equal(vec![0, 1, 2]));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::smallest_equal(vec![4, 3, 2, 1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            -1,
            Solution::smallest_equal(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            21,
            Solution::smallest_equal(vec![
                7, 8, 3, 5, 2, 6, 3, 1, 1, 4, 5, 4, 8, 7, 2, 0, 9, 9, 0, 5, 7, 1, 6
            ])
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(
            21,
            Solution::smallest_equal_cycle(vec![
                7, 8, 3, 5, 2, 6, 3, 1, 1, 4, 5, 4, 8, 7, 2, 0, 9, 9, 0, 5, 7, 1, 6
            ])
        );
    }
}
