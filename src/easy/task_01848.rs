// 1848. Minimum Distance to the Target Element
// https://leetcode.com/problems/minimum-distance-to-the-target-element/

use crate::Solution;

impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let mut min_dist = std::i32::MAX;
        for (i, val) in nums.iter().enumerate() {
            if *val == target {
                min_dist = min_dist.min((i as i32 - start).abs());
                if min_dist == 0 {
                    break;
                }
            }
        }
        min_dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::get_min_distance(vec![1, 2, 3, 4, 5], 5, 3));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::get_min_distance(vec![1], 1, 0));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            0,
            Solution::get_min_distance(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1, 0)
        );
    }
}
