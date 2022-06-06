// 2293. Min Max Game
// https://leetcode.com/problems/min-max-game/

use crate::Solution;

impl Solution {
    pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
        while nums.len() > 1 {
            nums = nums
                .chunks(2)
                .enumerate()
                .map(|(i, pair)| match i % 2 {
                    0 => *pair.iter().min().unwrap(),
                    _ => *pair.iter().max().unwrap(),
                })
                .collect();
        }
        nums[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::min_max_game(vec![1, 3, 5, 2, 4, 8, 2, 2]), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::min_max_game(vec![3]), 3);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::min_max_game(vec![2, 5]), 2);
    }
}
